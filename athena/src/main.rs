use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
        <PageSectionGroup>
            <PageSection
                r#type={PageSectionType::Default}
                variant={PageSectionVariant::Light}
                limit_width=true
                sticky={[PageSectionSticky::Top]}
            >
                <Content>
                    <Title size={Size::XXXXLarge}>
                        { "Athena" }
                    </Title>
                </Content>
            </PageSection>
        </PageSectionGroup>

        <div style="padding: 32px;">
        <Flex>
            <FlexItem>
                <Card>
                    <CardTitle>{"Agents"}</CardTitle>

                    <CardBody>
                        <TextInputGroup>
                            <TextInputGroupMain
                                placeholder="agent name"
                                icon={Icon::Search}
                            />
                            <TextInputGroupUtilities>
                                <Button icon={Icon::Times} variant={ButtonVariant::Plain} />
                            </TextInputGroupUtilities>
                        </TextInputGroup>
                        <Divider r#type={DividerType::Hr} />
                    </CardBody>
                </Card>
            </FlexItem>
        </Flex>
        </div>

        <Content>
        </Content>
    </>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
