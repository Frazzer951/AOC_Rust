import json
import os
import re
import subprocess
from datetime import datetime
from typing import Any


def run_rust_tests() -> list[dict[str, Any]] | None:
    # Command to run Rust tests with JSON output
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
        result = subprocess.run(command, capture_output=True, text=True, check=True)
        json_output = result.stdout
        parsed_results = [json.loads(output) for output in json_output.split("\n") if output]

        return parsed_results
    except subprocess.CalledProcessError as e:
        print(f"An error occurred while running the tests: {e}")
        return None
    except json.JSONDecodeError as e:
        print(f"An error occurred while parsing the JSON output: {e}")
        return None


def filter_tests(test: dict[str, Any]) -> bool:
    return test.get("type") == "test" and test.get("event") != "started"


def parse_test_results() -> dict[int, dict[int, dict[int, list[bool]]]]:
    test_results = run_rust_tests()
    test_results = list(filter(filter_tests, test_results))
    results: dict[int, dict[int, dict[int, list[bool]]]] = {}

    for test in test_results:
        name = test["name"].split("::")
        result = test["event"]

        try:
            year = int(name[0][1:])
            day = int(name[1][4:])
            part = int(name[3][6:7])
        except Exception as e:
            print(f'Failed to parse `{test["name"]}`: {str(e)}')
            continue

        if year not in results:
            results[year] = {}

        if day not in results[year]:
            results[year][day] = {1: [], 2: []}

        results[year][day][part].append(result == "ok")
    return results


def aoc_status(
    results: dict[int, dict[int, dict[int, list[bool]]]],
) -> dict[int, dict[int, tuple[bool, bool]]]:
    aoc_problems = {}

    start_year = 2015
    start_day = 1

    current_date = datetime.now()
    year = current_date.year
    month = current_date.month
    day = current_date.day

    if month == 12:
        end_year = year
        end_day = day
    else:
        end_year = year - 1
        end_day = 25

    for year in range(start_year, end_year + 1):
        _start_day = start_day if year == start_year else 1
        _end_day = end_day if year == end_year else 25
        for day in range(_start_day, _end_day + 1):
            status = results.get(year, {}).get(day, {})
            p1 = status.get(1, [])
            p2 = status.get(2, [])
            p1_status = len(p1) > 0 and all(p1)
            p2_status = len(p2) > 0 and all(p2)

            if year not in aoc_problems:
                aoc_problems[year] = {}
            aoc_problems[year][day] = (p1_status, p2_status)
    return aoc_problems


def group_year_stats(
    year_stats: dict[int, dict[int, tuple[bool, bool]]],
) -> tuple[
    dict[int, dict[int, tuple[bool, bool]]],
    dict[int, dict[int, tuple[bool, bool]]],
    dict[int, dict[int, tuple[bool, bool]]],
]:
    completed = {}
    in_progress = {}
    not_started = {}

    for year, days in year_stats.items():
        all_completed = True
        any_completed = False
        for _day, (p1, p2) in days.items():
            if p1 or p2:
                any_completed = True
            if not (p1 and p2):
                all_completed = False

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
    stars = 0
    for _year, days in year_stats.items():
        for _day, (p1, p2) in days.items():
            if p1:
                stars += 1
            if p2:
                stars += 1

    return stars


def create_year_overview(days: dict[int, tuple[bool, bool]]) -> list[str]:
    text: list[str] = [
        "| day   | part one | part two |",
        "| :---: | :------: | :------: |",
    ]

    for day, parts in days.items():
        part_one = "⭐️" if parts[0] else "❌"
        part_two = "⭐️" if parts[1] else "❌"
        text.append(f"| {day:02} | {part_one} | {part_two} |")

    return text


def gen_readme_text(
    in_progress: dict[int, dict[int, tuple[bool, bool]]],
    completed: dict[int, dict[int, tuple[bool, bool]]],
    not_started: dict[int, dict[int, tuple[bool, bool]]],
) -> str:
    text = ["## In Progess ✅"]
    for year, days in in_progress.items():
        text.append(f"### {year}")
        text.append(f"<details><summary>Days for {year}</summary>")
        text.append("<p>")
        text.append("")
        text = text + create_year_overview(days)
        text.append("")
        text.append("</p>")
        text.append("</details>")
        text.append("")

    if completed:
        text.append("")

        text.append("## Completed ⭐️")
        for year, days in completed.items():
            text.append(f"### {year}")
            text.append(f"<details><summary>Days for {year}</summary>")
            text.append("<p>")
            text.append("")
            text = text + create_year_overview(days)
            text.append("")
            text.append("</p>")
            text.append("</details>")
            text.append("")

    if not_started:
        text.append("")

        text.append("## Not Started ❌")
        for year, days in not_started.items():
            text.append(f"### {year}")
            text.append(f"<details><summary>Days for {year}</summary>")
            text.append("<p>")
            text.append("")
            text = text + create_year_overview(days)
            text.append("")
            text.append("</p>")
            text.append("</details>")
            text.append("")

    return "\n".join(text)


def replace_between_tags(string: str, content: str, start: str, end: str) -> str:
    content = "\n".join([start, content, end])

    return re.sub(
        pattern=rf"{start}.*?{end}",
        repl=content,
        string=string,
        flags=re.DOTALL,
    )


def update_stars(readme: str, star_count: int) -> str:
    return re.sub(
        pattern=r"&message=\d+",
        repl=f"&message={star_count}",
        string=readme,
        flags=re.DOTALL,
    )


def update_stars_in_image(current_dir: str, star_count: int):
    image_dark = os.path.join(current_dir, "../image_dark.svg")
    image_light = os.path.join(current_dir, "../image_light.svg")
    content = f'				<span class="star-count">{star_count}</span>'

    with open(image_dark, encoding="UTF-8") as f:
        svg_content = f.read()

    svg_content = replace_between_tags(svg_content, content, "<!-- start star count -->", "<!-- end star count -->")

    with open(image_dark, "w", encoding="UTF-8") as f:
        f.write(svg_content)

    with open(image_light, encoding="UTF-8") as f:
        svg_content = f.read()

    svg_content = replace_between_tags(svg_content, content, "<!-- start star count -->", "<!-- end star count -->")

    with open(image_light, "w", encoding="UTF-8") as f:
        f.write(svg_content)


def update_readme(text: str, stars: int):
    current_dir = os.path.dirname(os.path.abspath(__file__))
    path = os.path.join(current_dir, "../readme.md")
    readme_file = os.path.abspath(path)

    with open(readme_file, encoding="UTF-8") as f:
        current_readme = f.read()

    readme = replace_between_tags(
        current_readme,
        text,
        "<!-- start completed section -->",
        "<!-- end completed section -->",
    )
    readme = update_stars(readme, stars)

    with open(readme_file, "w", encoding="UTF-8") as f:
        f.write(readme)

    update_stars_in_image(current_dir, stars)


if __name__ == "__main__":
    results = parse_test_results()

    year_stats = aoc_status(results)

    in_progress, completed, not_started = group_year_stats(year_stats)
    total_stars = get_stars(year_stats)

    print("Completed years:")
    print(completed.keys())
    print("\nIn progress years:")
    print(in_progress.keys())
    print("\nNot started years:")
    print(not_started.keys())

    print("Total Stars:", total_stars)

    readme_text = gen_readme_text(in_progress, completed, not_started)

    update_readme(readme_text, total_stars)
