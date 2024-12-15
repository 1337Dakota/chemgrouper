#!/usr/bin/fish

set TARGETS "https://raw.githubusercontent.com/space-wizards/space-station-14/refs/heads/master/Resources/Prototypes/Recipes/Reactions/chemicals.yml" "https://raw.githubusercontent.com/space-wizards/space-station-14/refs/heads/master/Resources/Prototypes/Recipes/Reactions/cleaning.yml" "https://raw.githubusercontent.com/space-wizards/space-station-14/refs/heads/master/Resources/Prototypes/Recipes/Reactions/fun.yml" "https://raw.githubusercontent.com/space-wizards/space-station-14/refs/heads/master/Resources/Prototypes/Recipes/Reactions/medicine.yml" "https://raw.githubusercontent.com/space-wizards/space-station-14/refs/heads/master/Resources/Prototypes/Recipes/Reactions/pyrotechnic.yml"
#set TARGETS "https://raw.githubusercontent.com/space-wizards/space-station-14/refs/heads/master/Resources/Prototypes/Recipes/Reactions/pyrotechnic.yml"
set OUT_FILE out.json
set TEMP_FILE tmp

for target in $TARGETS
    wget -q -O - "$target" >> $TEMP_FILE
end

sd "$(echo -e "\xEF\xBB\xBF")" '' $TEMP_FILE

yq --indent 4 '[.[] | select(.source == null and .products != null) | {
        "name": .id,
        "deps": (.reactants | to_entries | map({key: .key, value: .value.amount}) | from_entries),
        "reaction_temp": .minTemp}]' $TEMP_FILE > $OUT_FILE

rm $TEMP_FILE