import init, { synthesize_with_annotation } from '../pkg/pearl_regex_core'

async function main() {
  await init()
  const positives = ["Buy milk", "Buy eggs"]
  const negatives = ["Call mom"]
  const result = synthesize_with_annotation(positives, negatives)
  console.log("Generated pattern:", result)
  document.body.innerHTML = `<pre>Generated: ${result}</pre>`
}

main()
