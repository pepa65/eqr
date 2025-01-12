
use builtin;
use str;

set edit:completion:arg-completer[eqr] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'eqr'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'eqr'= {
            cand -o 'Output file (jpg/png/svg) [default: qr.png]'
            cand --output 'Output file (jpg/png/svg) [default: qr.png]'
            cand -l 'QR error correction level (L: 7%, M: 15%, Q: 25%, H: 30%)'
            cand --level 'QR error correction level (L: 7%, M: 15%, Q: 25%, H: 30%)'
            cand -e 'Edge size (in unit blocks)'
            cand --edge 'Edge size (in unit blocks)'
            cand -f 'Foreground RGB color (hex code)'
            cand --fg 'Foreground RGB color (hex code)'
            cand -b 'Background RGB color (hex code)'
            cand --bg 'Background RGB color (hex code)'
            cand -s 'Scale factor (1..255)'
            cand --scale 'Scale factor (1..255)'
            cand -t 'Output to terminal'
            cand --terminal 'Output to terminal'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
    ]
    $completions[$command]
}
