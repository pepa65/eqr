_qr() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            "$1")
                cmd="qr"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        qr)
            opts="-h -V -o -f -b -B -L -s --help --version --output --fg --bg --border --error-correction-level --scale <STRING>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fg)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --bg)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -b)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --border)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -B)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --error-correction-level)
                    COMPREPLY=($(compgen -W "low medium quartile high" -- "${cur}"))
                    return 0
                    ;;
                -L)
                    COMPREPLY=($(compgen -W "low medium quartile high" -- "${cur}"))
                    return 0
                    ;;
                --scale)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _qr -o bashdefault -o default qr
