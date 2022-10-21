pub struct Test {
    env: Env,
}

pub struct EnvConfig {

}

pub struct Env {
    agents: Vec<Agent>,

    config: EnvConfig,
}

pub struct AgentConfig {
    is_active: bool,
}

pub struct Agent {
    name: String,
    sequencer: Sequencer,
    driver: Driver,
    monitor: Monitor,

    config: AgentConfig,
}

pub struct Sequence {

}

pub struct Sequencer {
    sequences: Vec<Sequence>,
}

pub struct Driver {

}

pub struct Monitor {

}
