complete -c eqr -s o -l output -d 'Output file (jpg/png/svg) [default: qr.png]' -r
complete -c eqr -s l -l level -d 'QR error correction level (L: 7%, M: 15%, Q: 25%, H: 30%)' -r
complete -c eqr -s p -l path -d 'Path to logo (png/jpg)' -r -F
complete -c eqr -s e -l edge -d 'Edge size (in unit blocks)' -r
complete -c eqr -s f -l fg -d 'Foreground RGB color (hex code)' -r
complete -c eqr -s b -l bg -d 'Background RGB color (hex code)' -r
complete -c eqr -s s -l scale -d 'Size of unit block in pixels (1..255)' -r
complete -c eqr -s t -l terminal -d 'Output to terminal (never the logo)'
complete -c eqr -s h -l help -d 'Print help'
complete -c eqr -s V -l version -d 'Print version'
