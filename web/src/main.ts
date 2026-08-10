import init, { synthesize } from '../pkg/pearl_regex_core'

async function main() {
  await init()
  const result = synthesize(["Buy milk", "Call mom"])
  console.log("Generated pattern:", result)
  document.body.innerHTML = `<pre>Generated: ${result}</pre>`
}

main()
