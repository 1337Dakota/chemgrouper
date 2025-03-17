from pathlib import Path
import subprocess
import os

subprocess.run(["python", "parse_13.py"])
subprocess.run(["fish", "./parse_14.fish"])

chem13 = Path("out_13.json").read_text()
chem13_name = "MonkeStation"
chem14 = Path("out_14.json").read_text()
chem14_name = "WizDen"

subprocess.run(["fish", "./parse_14.fish", "funky-station/funky-station", "https://raw.githubusercontent.com/funky-station/funky-station/refs/heads/master/Resources/Prototypes/_Funkystation/Recipes/Reactions/exotic.yml", "https://raw.githubusercontent.com/funky-station/funky-station/refs/heads/master/Resources/Prototypes/_Funkystation/Recipes/Reactions/medicine.yml", "https://raw.githubusercontent.com/funky-station/funky-station/refs/heads/master/Resources/Prototypes/_Funkystation/Recipes/Reactions/toxins.yml"])
chemFunky = Path("out_14.json").read_text()
chemFunky_name = "FunkyStation"


out = "{\"" + chem13_name + "\":" + chem13 + ",\"" + chem14_name + "\":" + chem14 + ",\"" + chemFunky_name + "\":" + chemFunky + "}"

with open("temp.json", "w") as file:
    file.write(out)

with open("out.json", "w") as file:
    subprocess.run(["jq", ".", "--indent", "4", "temp.json"], stdout=file, check=True)

#os.remove("temp.json")
#os.remove("out_13.json")
#os.remove("out_14.json")
