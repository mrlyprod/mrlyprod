import { Box, Button, Caption, Cluster, Stack, Title } from "mrlyui"
import { HELP, LEAD } from "../lib/site"

export function Panel({ designed }: { designed: boolean }) {
  return (
    <Box>
      <Stack tight>
        <Title>{designed ? "Order yours." : "Coming soon."}</Title>
        <Caption>{designed ? LEAD : "Want it sooner?"}</Caption>
        <Cluster>
          <Button href={HELP} primary>
            {designed ? "Get in touch" : "Say hello"}
          </Button>
        </Cluster>
      </Stack>
    </Box>
  )
}
