
use builtin;
use str;

set edit:completion:arg-completer[qr] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'qr'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'qr'= {
            cand -o 'Output file (supported file extensions: jpg, png, svg); omit to print QR code to console'
            cand --output 'Output file (supported file extensions: jpg, png, svg); omit to print QR code to console'
            cand -f 'Foreground RGB color (hex code)'
            cand --fg 'Foreground RGB color (hex code)'
            cand -b 'Background RGB color (hex code)'
            cand --bg 'Background RGB color (hex code)'
            cand -B 'Border size (expressed in unit blocks)'
            cand --border 'Border size (expressed in unit blocks)'
            cand -L 'QR error correction level'
            cand --error-correction-level 'QR error correction level'
            cand -s 'Scale factor (1..255)'
            cand --scale 'Scale factor (1..255)'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
        }
    ]
    $completions[$command]
}
