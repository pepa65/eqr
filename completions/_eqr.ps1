
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'eqr' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'eqr'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'eqr' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Output file (jpg/png/svg) [default: qr.png]')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output file (jpg/png/svg) [default: qr.png]')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'QR error correction level (L: 7%, M: 15%, Q: 25%, H: 30%)')
            [CompletionResult]::new('--level', '--level', [CompletionResultType]::ParameterName, 'QR error correction level (L: 7%, M: 15%, Q: 25%, H: 30%)')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Path to logo (png/jpg)')
            [CompletionResult]::new('--path', '--path', [CompletionResultType]::ParameterName, 'Path to logo (png/jpg)')
            [CompletionResult]::new('-P', '-P ', [CompletionResultType]::ParameterName, 'Logo proportion to the whole image (0..1)')
            [CompletionResult]::new('--proportion', '--proportion', [CompletionResultType]::ParameterName, 'Logo proportion to the whole image (0..1)')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Edge size (in unit blocks)')
            [CompletionResult]::new('--edge', '--edge', [CompletionResultType]::ParameterName, 'Edge size (in unit blocks)')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Foreground RGB color (hex code)')
            [CompletionResult]::new('--fg', '--fg', [CompletionResultType]::ParameterName, 'Foreground RGB color (hex code)')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'Background RGB color (hex code)')
            [CompletionResult]::new('--bg', '--bg', [CompletionResultType]::ParameterName, 'Background RGB color (hex code)')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Size of unit block in pixels (1..255)')
            [CompletionResult]::new('--scale', '--scale', [CompletionResultType]::ParameterName, 'Size of unit block in pixels (1..255)')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Output to terminal (never the logo)')
            [CompletionResult]::new('--terminal', '--terminal', [CompletionResultType]::ParameterName, 'Output to terminal (never the logo)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
