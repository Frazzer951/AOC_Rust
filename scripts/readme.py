import json
import logging
import re
import subprocess
from datetime import datetime
from pathlib import Path
from typing import Any

# Constants
START_YEAR = 2015
START_DAY = 1

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


def run_rust_tests() -> list[dict[str, Any]] | None:
    """Run Rust tests and return parsed JSON output."""
    command = [
        "cargo",
        "+nightly",
        "test",
        "--",
        "-Z",
        "unstable-options",
        "--format",
        "json",
    ]

    try:
        result = subprocess.run(command, capture_output=True, text=True)
        json_output = result.stdout
        return [json.loads(output) for output in json_output.split("\n") if output]
    except subprocess.CalledProcessError as e:
        logger.error(f"An error occurred while running the tests: {e}")
    except json.JSONDecodeError as e:
        logger.error(f"An error occurred while parsing the JSON output: {e}")
    return None


def filter_tests(test: dict[str, Any]) -> bool:
    """Filter test results."""
    return test.get("type") == "test" and test.get("event") != "started"


def parse_test_name(name: str) -> tuple[int, int, int]:
    """Parse test name to extract year, day, and part."""
    parts = name.split("::")
    return int(parts[0][1:]), int(parts[1][4:]), int(parts[3][6:7])


def parse_test_results() -> dict[int, dict[int, dict[int, list[bool]]]]:
    """Parse test results into a structured dictionary."""
    test_results = run_rust_tests()
    if not test_results:
        return {}

    results: dict[int, dict[int, dict[int, list[bool]]]] = {}
    for test in filter(filter_tests, test_results):
        try:
            year, day, part = parse_test_name(test["name"])
            results.setdefault(year, {}).setdefault(day, {}).setdefault(part, []).append(test["event"] == "ok")
        except Exception as e:
            logger.error(f'Failed to parse `{test["name"]}`: {str(e)}')
    return results


def aoc_status(
    results: dict[int, dict[int, dict[int, list[bool]]]],
) -> dict[int, dict[int, tuple[bool, bool]]]:
    """Determine AoC problem status for each year and day."""
    aoc_problems = {}
    current_date = datetime.now()
    end_year, end_day = (
        (current_date.year, current_date.day) if current_date.month == 12 else (current_date.year - 1, 25)
    )

    for year in range(START_YEAR, end_year + 1):
        start_day = START_DAY if year == START_YEAR else 1
        end_day_for_year = end_day if year == end_year else 25
        for day in range(start_day, end_day_for_year + 1):
            status = results.get(year, {}).get(day, {})
            p1 = status.get(1, [])
            p2 = status.get(2, [])
            aoc_problems.setdefault(year, {})[day] = (bool(p1 and all(p1)), bool(p2 and all(p2)))
    return aoc_problems


def group_year_stats(
    year_stats: dict[int, dict[int, tuple[bool, bool]]],
) -> tuple[
    dict[int, dict[int, tuple[bool, bool]]],
    dict[int, dict[int, tuple[bool, bool]]],
    dict[int, dict[int, tuple[bool, bool]]],
]:
    """Group year statistics into completed, in progress, and not started."""
    completed, in_progress, not_started = {}, {}, {}

    for year, days in year_stats.items():
        all_completed = all(p1 and p2 for p1, p2 in days.values())
        any_completed = any(p1 or p2 for p1, p2 in days.values())

        if all_completed:
            completed[year] = days
        elif any_completed:
            in_progress[year] = days
        else:
            not_started[year] = days

    return in_progress, completed, not_started


def get_stars(
    year_stats: dict[int, dict[int, tuple[bool, bool]]],
) -> int:
    """Calculate total number of stars earned."""
    return sum(sum(p1 + p2 for p1, p2 in days.values()) for days in year_stats.values())


def create_year_overview(days: dict[int, tuple[bool, bool]]) -> list[str]:
    """Create a markdown table overview for a year's progress."""
    text = [
        "| day   | part one | part two |",
        "| :---: | :------: | :------: |",
    ]
    for day, (p1, p2) in days.items():
        text.append(f"| {day:02} | {'⭐️' if p1 else '❌'} | {'⭐️' if p2 else '❌'} |")
    return text


def gen_readme_text(
    in_progress: dict[int, dict[int, tuple[bool, bool]]],
    completed: dict[int, dict[int, tuple[bool, bool]]],
    not_started: dict[int, dict[int, tuple[bool, bool]]],
) -> str:
    """Generate README text with progress overview."""
    sections = [
        ("## In Progress ✅", in_progress),
        ("## Completed ⭐️", completed),
        ("## Not Started ❌", not_started),
    ]

    text = []
    for header, data in sections:
        if data:
            text.append(header)
            for year, days in data.items():
                text.extend(
                    [
                        f"### {year}",
                        f"<details><summary>Days for {year}</summary>",
                        "<p>",
                        "",
                        *create_year_overview(days),
                        "",
                        "</p>",
                        "</details>",
                        "",
                    ]
                )

    return "\n".join(text)


def replace_between_tags(string: str, content: str, start: str, end: str) -> str:
    """Replace content between start and end tags in a string."""
    content = f"{start}\n{content}\n{end}"
    return re.sub(
        pattern=rf"{re.escape(start)}.*?{re.escape(end)}",
        repl=content,
        string=string,
        flags=re.DOTALL,
    )


def update_stars(readme: str, star_count: int) -> str:
    """Update star count in README."""
    return re.sub(
        pattern=r"&message=\d+",
        repl=f"&message={star_count}",
        string=readme,
        flags=re.DOTALL,
    )


def update_stars_in_image(current_dir: Path, star_count: int):
    """Update star count in SVG images."""
    for image_file in ["image_dark.svg", "image_light.svg"]:
        image_path = current_dir.parent / image_file
        content = f'				<span class="star-count">{star_count}</span>'

        with image_path.open(encoding="UTF-8") as f:
            svg_content = f.read()

        svg_content = replace_between_tags(svg_content, content, "<!-- start star count -->", "<!-- end star count -->")

        with image_path.open("w", encoding="UTF-8") as f:
            f.write(svg_content)


def update_readme(text: str, stars: int):
    """Update README file with new content and star count."""
    current_dir = Path(__file__).parent
    readme_file = current_dir.parent / "readme.md"

    with readme_file.open(encoding="UTF-8") as f:
        current_readme = f.read()

    readme = replace_between_tags(
        current_readme,
        text,
        "<!-- start completed section -->",
        "<!-- end completed section -->",
    )
    readme = update_stars(readme, stars)

    with readme_file.open("w", encoding="UTF-8") as f:
        f.write(readme)

    update_stars_in_image(current_dir, stars)


if __name__ == "__main__":
    results = parse_test_results()
    year_stats = aoc_status(results)
    in_progress, completed, not_started = group_year_stats(year_stats)
    total_stars = get_stars(year_stats)

    logger.info(f"Completed years: {list(completed.keys())}")
    logger.info(f"In progress years: {list(in_progress.keys())}")
    logger.info(f"Not started years: {list(not_started.keys())}")
    logger.info(f"Total Stars: {total_stars}")

    readme_text = gen_readme_text(in_progress, completed, not_started)
    update_readme(readme_text, total_stars)
