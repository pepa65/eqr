
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
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Output file (supported file extensions: jpg, png, svg); omit to print QR code to console')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'Output file (supported file extensions: jpg, png, svg); omit to print QR code to console')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'Foreground RGB color (hex code)')
            [CompletionResult]::new('--fg', 'fg', [CompletionResultType]::ParameterName, 'Foreground RGB color (hex code)')
            [CompletionResult]::new('-b', 'b', [CompletionResultType]::ParameterName, 'Background RGB color (hex code)')
            [CompletionResult]::new('--bg', 'bg', [CompletionResultType]::ParameterName, 'Background RGB color (hex code)')
            [CompletionResult]::new('-B', 'B', [CompletionResultType]::ParameterName, 'Border size (expressed in unit blocks)')
            [CompletionResult]::new('--border', 'border', [CompletionResultType]::ParameterName, 'Border size (expressed in unit blocks)')
            [CompletionResult]::new('-L', 'L', [CompletionResultType]::ParameterName, 'QR error correction level')
            [CompletionResult]::new('--error-correction-level', 'error-correction-level', [CompletionResultType]::ParameterName, 'QR error correction level')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Scale factor (1..255)')
            [CompletionResult]::new('--scale', 'scale', [CompletionResultType]::ParameterName, 'Scale factor (1..255)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
