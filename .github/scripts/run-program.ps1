# Runs a program, which may be a windowed one, waits for it, and says what it said. A program that has hung
# takes no longer than this, where it would otherwise use up the whole job and leave nothing to go on.
function Run-Program([string]$Program, [string[]]$Arguments, [double]$Minutes = 20) {
    $info = [Diagnostics.ProcessStartInfo]::new($Program)
    foreach ($argument in $Arguments) { $info.ArgumentList.Add($argument) }
    $info.UseShellExecute = $false
    $info.RedirectStandardOutput = $true
    $info.RedirectStandardError = $true
    $process = [Diagnostics.Process]::Start($info)
    $out = $process.StandardOutput.ReadToEndAsync()
    $err = $process.StandardError.ReadToEndAsync()
    $finished = $process.WaitForExit([int]($Minutes * 60000))
    if (-not $finished) { $process.Kill($true) }
    $process.WaitForExit()
    # Something it left running can keep these open after it has gone, and has nothing to add to them.
    $read = [Threading.Tasks.Task]::WaitAll([Threading.Tasks.Task[]]@($out, $err), 60000)
    $said = $(if ($read) { $out.Result + $err.Result } else { '(something it started is still holding what it said)' })
    if (-not $finished) {
        return [pscustomobject]@{ Code = -1; Output = "It was still running after $Minutes minutes, so it was stopped.`n$said" }
    }
    return [pscustomobject]@{ Code = $process.ExitCode; Output = $said }
}
