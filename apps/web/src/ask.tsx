import { useSyncExternalStore } from "react"
import { Button, Cluster, Modal, Text } from "mrlyui"

type Question = { text: string; resolve: (ok: boolean) => void }

let asked: Question | null = null
const subs = new Set<() => void>()

function keep(next: Question | null): void {
  asked = next
  for (const sub of subs) sub()
}

export function ask(text: string): Promise<boolean> {
  return new Promise(resolve => {
    asked?.resolve(false)
    keep({ text, resolve })
  })
}

function answer(ok: boolean): void {
  const done = asked
  keep(null)
  done?.resolve(ok)
}

export function Asker() {
  const question = useSyncExternalStore(
    sub => {
      subs.add(sub)
      return () => subs.delete(sub)
    },
    () => asked,
    () => null,
  )

  return (
    <Modal open={question !== null} onClose={() => answer(false)}>
      <Text>{question?.text ?? ""}</Text>
      <Cluster>
        <Button primary onClick={() => answer(true)}>yes</Button>
        <Button onClick={() => answer(false)}>no</Button>
      </Cluster>
    </Modal>
  )
}
