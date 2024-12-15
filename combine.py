# print("NOT IMPLEMENTED, DO IT YOURSELF! (I am lazy)")
# exit()

from pathlib import Path
import subprocess
import os

chem13 = Path("out_13.json").read_text()
chem13_name = "MonkeStation"
chem14 = Path("out_14.json").read_text()
chem14_name = "WizDen"

out = '{"' + chem13_name + '":' + chem13 + "," + '"' + chem14_name + '":' + chem14 + "}"

with open("temp.json", "w") as file:
    file.write(out)

with open("out.json", "w") as file:
    subprocess.run(["jq", ".", "--indent", "4", "temp.json"], stdout=file, check=True)

os.remove("temp.json")
