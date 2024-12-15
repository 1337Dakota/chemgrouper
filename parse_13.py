import re
import json
import requests


# Define a function to parse each chemical construct
def parse_chemical_reaction(data):
    # Extracting the name (last part after the last slash) and remove comments
    name_match = (
        data.split("/datum/chemical_reaction/")[-1].split("\n")[0].strip()
    )  # Get the part after the last slash
    name = name_match.split("//")[
        0
    ].strip()  # Remove comment after "//" and trim whitespace

    # Extracting the amount from the results list (correctly from the results line only)
    results_match = re.search(r"results = list\(/datum/reagent/[\w/]+ = (\d+)\)", data)
    amount = int(results_match.group(1)) if results_match else None

    # Extracting required reagents and amounts from the correct line
    required_reagents_line_match = re.search(
        r"required_reagents = list\(([^)]+)\)", data
    )
    required_reagents = {}

    if required_reagents_line_match:
        required_reagents_str = required_reagents_line_match.group(1)
        required_reagents_match = re.findall(
            r"/datum/reagent/([\w/]+) = (\d+)", required_reagents_str
        )
        required_reagents = {
            re.sub(r".*/", "", reagent).capitalize(): int(amount)
            for reagent, amount in required_reagents_match
        }

    # Only return if both amount and deps are set
    if amount is not None and required_reagents:
        return {
            "name": name.split("/")[
                -1
            ].capitalize(),  # Correctly handle the name after the last slash
            "deps": required_reagents,
            "reaction_temp": None,
            "amount": amount,
        }
    return None


# Function to fetch data from a list of URLs
def fetch_data_from_urls(urls):
    parsed_reactions = []

    for url in urls:
        old_len = len(parsed_reactions)
        try:
            print(f"Fetching data from {url}...")
            response = requests.get(url)
            response.raise_for_status()  # Raise an exception for 4xx/5xx responses
            content = response.text  # Get the raw content

            # Split the content into individual constructs
            constructs = content.split("/datum/chemical_reaction")

            # Parse each construct and add to the list if valid
            for construct in constructs[1:]:  # Skip the first split since it's empty
                full_construct = (
                    "/datum/chemical_reaction" + construct
                )  # Add back the delimiter
                reaction = parse_chemical_reaction(full_construct)
                if reaction:  # Filter out None results
                    parsed_reactions.append(reaction)

            print(f"Parsed {len(parsed_reactions) - old_len} new Reactions")
        except requests.RequestException as e:
            print(f"Error fetching {url}: {e}")

    return parsed_reactions


# List of URLs to fetch data from
urls = [
    "https://raw.githubusercontent.com/Monkestation/Monkestation2.0/refs/heads/master/code/modules/reagents/chemistry/recipes/drugs.dm",
    "https://raw.githubusercontent.com/Monkestation/Monkestation2.0/refs/heads/master/code/modules/reagents/chemistry/recipes/medicine.dm",
    "https://raw.githubusercontent.com/Monkestation/Monkestation2.0/refs/heads/master/code/modules/reagents/chemistry/recipes/others.dm",
    "https://raw.githubusercontent.com/Monkestation/Monkestation2.0/refs/heads/master/code/modules/reagents/chemistry/recipes/pyrotechnics.dm",
    "https://raw.githubusercontent.com/Monkestation/Monkestation2.0/refs/heads/master/code/modules/reagents/chemistry/recipes/toxins.dm",
]

# Fetch and parse data from URLs
parsed_reactions = fetch_data_from_urls(urls)

# Write the results to a JSON file
output_file = "out_13.json"
with open(output_file, "w") as file:
    json.dump(parsed_reactions, file, indent=4)

print(
    f"Parsed {len(parsed_reactions)} valid chemical reactions and saved to '{output_file}'."
)
