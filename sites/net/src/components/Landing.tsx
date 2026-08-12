import { Box, Button, Caption, Cluster, Grid, Section, Stack, Text, Title } from "mrlyui"
import type { Site } from "../lib/data"
import { FUNNEL } from "../lib/site"

export function Landing({ site }: { site: Site }) {
  return (
    <>
      <Text className="lead">{site.home[1]}</Text>
      <Section label="start">
        <Cluster>
          {FUNNEL.map(([name, href], at) => (
            <Button key={name} href={href} primary={at === 0}>
              {name}
            </Button>
          ))}
        </Cluster>
      </Section>
      <Section label="products">
        <Grid min={220}>
          {Object.entries(site.products).map(([route, [title, desc]]) => (
            <a key={route} href={`/${route}`}>
              <Box>
                <Stack tight>
                  <Title>{title}</Title>
                  <Caption>{desc}</Caption>
                </Stack>
              </Box>
            </a>
          ))}
        </Grid>
      </Section>
    </>
  )
}
