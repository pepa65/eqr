complete -c eqr -s o -l output -d 'Output file (jpg/png/svg), print to console if not given' -r -F
complete -c eqr -s l -l level -d 'QR error correction level (L: 7%, M: 15%, Q: 25%, H: 30%)' -r -f -a "{L	,low	,M	,medium	,Q	,quartile	,H	,high	}"
complete -c eqr -s e -l edge -d 'Edge size (in unit blocks)' -r
complete -c eqr -s f -l fg -d 'Foreground RGB color (hex code)' -r
complete -c eqr -s b -l bg -d 'Background RGB color (hex code)' -r
complete -c eqr -s s -l scale -d 'Scale factor (1..255)' -r
complete -c eqr -s h -l help -d 'Print help information'
complete -c eqr -s V -l version -d 'Print version information'
