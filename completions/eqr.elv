
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
            cand --qr-path 'Output file (jpg/png/svg) [default: qr.png]'
            cand -l 'QR error correction level (L: 7%, M: 15%, Q: 25%, H: 30%)'
            cand --level 'QR error correction level (L: 7%, M: 15%, Q: 25%, H: 30%)'
            cand -p 'Path to logo (png/jpg)'
            cand --path 'Path to logo (png/jpg)'
            cand -P 'Logo proportion to the whole image (0..1)'
            cand --proportion 'Logo proportion to the whole image (0..1)'
            cand -e 'Edge size (in unit blocks)'
            cand --edge 'Edge size (in unit blocks)'
            cand -f 'Foreground RGB color (hex code)'
            cand --fg 'Foreground RGB color (hex code)'
            cand -b 'Background RGB color (hex code)'
            cand --bg 'Background RGB color (hex code)'
            cand -s 'Size of unit block in pixels (1..255)'
            cand --scale 'Size of unit block in pixels (1..255)'
            cand -t 'Output to terminal (never the logo)'
            cand --terminal 'Output to terminal (never the logo)'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
    ]
    $completions[$command]
}
