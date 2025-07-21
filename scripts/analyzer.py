#!/usr/bin/env python
import json
from collections import defaultdict


def get_type_name(value):
    """Gets the type name of a value, with special handling for lists."""
    if isinstance(value, list):
        if not value:
            return "list(empty)"
        # Check the type of the first element to describe the list contents
        first_element_type = get_type_name(value[0])
        return f"list({first_element_type})"
    return type(value).__name__


def analyze_object_structure(obj, structure, total_count):
    """
    Recursively analyzes the structure of a dictionary (JSON object),
    counting key occurrences and types.
    """
    for key, value in obj.items():
        # Increment the count for this key
        structure[key]["count"] += 1

        # Get the type of the value
        type_name = get_type_name(value)

        # Store type information
        if "types" not in structure[key]:
            structure[key]["types"] = defaultdict(int)
        structure[key]["types"][type_name] += 1

        # If the value is a dictionary, recurse
        if isinstance(value, dict):
            if "children" not in structure[key]:
                # Initialize children structure
                structure[key]["children"] = defaultdict(lambda: defaultdict(int))

            # Special handling for retweeted_status: merge its children into the parent
            if key == "retweeted_status":
                # We just note the presence of retweeted_status, but analyze its
                # children as if they were part of the parent status.
                analyze_object_structure(value, structure, total_count)
            elif key == "pic_infos":
                # Special handling for pic_infos: analyze the values of the dict as a list
                for item in value.values():
                    if isinstance(item, dict):
                        analyze_object_structure(
                            item, structure[key]["children"], total_count
                        )
            else:
                # For other nested objects (like user, page_info), analyze them separately.
                analyze_object_structure(value, structure[key]["children"], total_count)

        # If the value is a list of dictionaries, analyze the structure of the objects in the list
        elif isinstance(value, list) and value and isinstance(value[0], dict):
            if "children" not in structure[key]:
                structure[key]["children"] = defaultdict(lambda: defaultdict(int))
            for item in value:
                if isinstance(item, dict):
                    analyze_object_structure(
                        item, structure[key]["children"], total_count
                    )


def main():
    """
    Main function to load data, run analysis, and save the result.
    """
    try:
        with open("full_favorites.json", "r", encoding="utf-8") as f:
            data = json.load(f)
    except FileNotFoundError:
        print("Error: full_favorites.json not found.")
        return
    except json.JSONDecodeError:
        print("Error: Could not decode JSON from full_favorites.json.")
        return

    if not isinstance(data, dict) or "favorites" not in data:
        print("Error: JSON structure is not as expected. 'favorites' key missing.")
        return

    favorites = data.get("favorites", [])
    if not isinstance(favorites, list):
        print("Error: 'favorites' should be a list.")
        return

    total_favorites = len(favorites)

    # Main structure to hold the analysis results
    # Using defaultdict for easier nested assignments
    status_structure = defaultdict(lambda: defaultdict(int))

    # Process each favorite's status
    for favorite in favorites:
        if "status" in favorite and isinstance(favorite["status"], dict):
            analyze_object_structure(
                favorite["status"], status_structure, total_favorites
            )

    # Convert defaultdicts to regular dicts for clean JSON output
    def finalize_structure(d, parent_count):
        final_dict = {}
        for key, value in d.items():
            node = {
                "count": value["count"],
                "frequency": f"{value['count'] / parent_count:.2%}",
                "types": {t: c for t, c in value["types"].items()},
            }
            if "children" in value and value["children"]:
                node["children"] = finalize_structure(value["children"], value["count"])
            final_dict[key] = node
        return final_dict

    final_result = {
        "total_posts": total_favorites,
        "post_structure": finalize_structure(status_structure, total_favorites),
    }

    # Save the result to a JSON file
    with open("analysis_result.json", "w", encoding="utf-8") as f:
        json.dump(final_result, f, indent=2, ensure_ascii=False)

    print("Analysis complete. Results saved to analysis_result.json")


if __name__ == "__main__":
    main()
