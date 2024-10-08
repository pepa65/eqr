complete -c eqr -s o -l output -d 'Output file (supported file extensions: jpg, png, svg); omit to print QR code to console' -r -F
complete -c eqr -s f -l fg -d 'Foreground RGB color (hex code)' -r
complete -c eqr -s b -l bg -d 'Background RGB color (hex code)' -r
complete -c eqr -s B -l border -d 'Border size (expressed in unit blocks)' -r
complete -c eqr -s L -l error-correction-level -d 'QR error correction level' -r -f -a "{low	,medium	,quartile	,high	}"
complete -c eqr -s s -l scale -d 'Scale factor (1..255)' -r
complete -c eqr -s h -l help -d 'Print help information'
complete -c eqr -s V -l version -d 'Print version information'
