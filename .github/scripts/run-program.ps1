# Runs a program, which may be a windowed one, waits for it, and says what it said. A program that has hung
# takes no longer than this, where it would otherwise use up the whole job and leave nothing to go on.
function Run-Program([string]$Program, [string[]]$Arguments, [double]$Minutes = 20) {
    $info = [Diagnostics.ProcessStartInfo]::new($Program)
    foreach ($argument in $Arguments) { $info.ArgumentList.Add($argument) }
    $info.UseShellExecute = $false
    $info.RedirectStandardOutput = $true
    $info.RedirectStandardError = $true
    $process = [Diagnostics.Process]::Start($info)
    $out = [IO.MemoryStream]::new()
    $err = [IO.MemoryStream]::new()
    $copying = [Threading.Tasks.Task[]]@(
        $process.StandardOutput.BaseStream.CopyToAsync($out),
        $process.StandardError.BaseStream.CopyToAsync($err)
    )
    $finished = $process.WaitForExit([int]($Minutes * 60000))
    if (-not $finished) { $process.Kill($true) }
    $process.WaitForExit()
    # Something it left running can keep these open after it has gone, and all it said by then is here.
    [void][Threading.Tasks.Task]::WaitAll($copying, 10000)
    $encoding = [Console]::OutputEncoding
    $said = $encoding.GetString($out.ToArray()) + $encoding.GetString($err.ToArray())
    if (-not $finished) {
        return [pscustomobject]@{ Code = -1; Output = "It was still running after $Minutes minutes, so it was stopped.`n$said" }
    }
    return [pscustomobject]@{ Code = $process.ExitCode; Output = $said }
}
