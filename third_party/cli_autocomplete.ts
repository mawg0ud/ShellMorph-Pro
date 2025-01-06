import fs from "fs";
import path from "path";

const COMMANDS = ["encrypt", "decrypt", "exec"];
const OPTIONS = ["--help", "--verbose", "--output"];

function generateAutocompleteScript(): string {
    return `
# ShellMorph CLI Autocomplete
_shellmorph_autocomplete() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    opts="${COMMANDS.join(" ")} ${OPTIONS.join(" ")}"

    if [[ ${cur} == -* ]]; then
        COMPREPLY=($(compgen -W "${OPTIONS.join(" ")}" -- $cur))
    else
        COMPREPLY=($(compgen -W "${COMMANDS.join(" ")}" -- $cur))
    fi
}

complete -F _shellmorph_autocomplete shellmorph
`;
}

function installAutocompleteScript() {
    const scriptPath = path.join(
        process.env.HOME || "",
        ".shellmorph_autocomplete.sh"
    );
    fs.writeFileSync(scriptPath, generateAutocompleteScript());
    console.log(`Autocomplete script written to: ${scriptPath}`);
    console.log(
        `Add the following to your .bashrc or .zshrc:\n\nsource ${scriptPath}\n`
    );
}

if (require.main === module) {
    installAutocompleteScript();
}
