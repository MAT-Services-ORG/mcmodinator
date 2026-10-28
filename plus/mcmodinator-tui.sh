#!/usr/bin/env bash

ver="1 LSH"
shopt -s nullglob

echo "MAT Modinator $ver TUI for Linux BASH"
echo

modinator_files=(mcmodinator-cli*.appimage)
if ((${#modinator_files[@]} == 0)); then
    echo "[ERROR] MCModinaotr (mcmodinator-cli*.appimage) not found in current folder."
    read -r -p "Press Enter to continue..."
    exit 1
fi
ModinatorFile=${modinator_files[-1]}

pause_for_user() {
    read -r -p "Press Enter to continue..."
}

while true; do
    echo "Choose an option:"
    echo "Build mod              -> m"
    echo "Build CPack            -> c"
    echo "Build mod (LEGACY)     -> ml"
    echo "Build CPack (LEGACY)   -> cl"
    echo "Print version          -> v"
    echo "Print documentation    -> d"
    echo "Cancel                 -> x"

    read -r -p "Choose an option => " INPUT

    case "$INPUT" in
        x)
            exit 0
            ;;
        m|c|ml|cl)
            read -r -p "Project path (Empty for current folder)  => " InputPath
            InputPath=${InputPath:-.}
            echo "$InputPath"

            read -r -p "Output path (Empty for current folder) => " OutputPath
            OutputPath=${OutputPath:-.}
            echo "$OutputPath"

            args=()
            case "$INPUT" in
                m)  args=(-mtrue) ;;
                c)  args=(-ctrue) ;;
                ml) args=(-mtrue -l) ;;
                cl) args=(-ctrue -l) ;;
            esac

            ./"$ModinatorFile" "${args[@]}" -p "$InputPath" -o "$OutputPath"
            pause_for_user
            exit
            ;;
        v)
            MCModinator TUI V.$ver
            ./"$ModinatorFile" -V
            pause_for_user
            ;;
        d)
            ./"$ModinatorFile" -d
            pause_for_user
            ;;
        *)
            if [[ -z "$INPUT" ]]; then
                continue
            fi
            echo "Invalid input. Retry."
            echo
            ;;
    esac
done