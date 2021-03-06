Set-PSDebug -Trace 1

$header_dir = 'C:\Program Files (x86)\National Instruments\Shared\ExternalCompilerSupport\C\include'
$llvm_inputs = '-I' + $header_dir;


$allow_pattern = 'NIS[yY][sS]C[fF][gG].*'
$allow_lists = '--allowlist-type', $allow_pattern, '--allowlist-function',$allow_pattern,'--allowlist-var', $allow_pattern

bindgen 'include/wrapper.h' -o 'src/bindings.rs' $allow_lists -- $llvm_inputs
