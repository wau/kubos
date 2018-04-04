use double;
//use failure::{Fail, Error};
use std::cell::RefCell;
use isis_ants_api::*;
use model::*;

mock_trait_no_default!(
		MockAntS,
		configure(KANTSController) -> AntSResult<()>,
		reset() -> AntSResult<()>,
		arm() -> AntSResult<()>,
		disarm() -> AntSResult<()>,
		deploy(KANTSAnt, bool, u8) -> AntSResult<()>,
		auto_deploy(u8) -> AntSResult<()>,
		cancel_deploy() -> AntSResult<()>,
		get_deploy() -> AntSResult<DeployStatus>,
		get_uptime() -> AntSResult<u32>,
		get_system_telemetry() -> AntSResult<AntsTelemetry>,
		get_activation_count(KANTSAnt) -> AntSResult<u8>,
		get_activation_time(KANTSAnt) -> AntSResult<u16>,
		watchdog_kick() -> AntSResult<()>,
		watchdog_start() -> AntSResult<()>,
		watchdog_stop() -> AntSResult<()>,
		passthrough(Vec<u8>, Vec<u8>) -> AntSResult<()>
	);

impl IAntS for MockAntS {
    fn new(
        _bus: KI2CNum,
        _primary: u8,
        _secondary: u8,
        _ant_count: u8,
        _timeout: u32,
    ) -> AntSResult<MockAntS> {
        Ok(MockAntS::new(
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(DeployStatus::default()),
            Ok(0),
            Ok(AntsTelemetry::default()),
            Ok(0),
            Ok(0),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        ))
    }
    mock_method!(configure(&self, config: KANTSController) -> AntSResult<()>);
    mock_method!(reset(&self) -> AntSResult<()>);
    mock_method!(arm(&self) -> AntSResult<()>);
    mock_method!(disarm(&self) -> AntSResult<()>);
    mock_method!(deploy(&self, antenna: KANTSAnt, force: bool, timeout: u8) -> AntSResult<()>);
    mock_method!(auto_deploy(&self, timeout: u8) -> AntSResult<()>);
    mock_method!(cancel_deploy(&self) -> AntSResult<()>);
    mock_method!(get_deploy(&self) -> AntSResult<DeployStatus>);
    mock_method!(get_uptime(&self) -> AntSResult<u32>);
    mock_method!(get_system_telemetry(&self) -> AntSResult<AntsTelemetry>);
    mock_method!(get_activation_count(&self, antenna: KANTSAnt) -> AntSResult<u8>);
    mock_method!(get_activation_time(&self, antenna: KANTSAnt) -> AntSResult<u16>);
    mock_method!(watchdog_kick(&self) -> AntSResult<()>);
    mock_method!(watchdog_start(&self) -> AntSResult<()>);
    mock_method!(watchdog_stop(&self) -> AntSResult<()>);
    mock_method!(passthrough(&self, tx: &[u8], rx: &mut [u8]) -> AntSResult<()>, self, {
            self.passthrough.call((tx.to_vec(), rx.to_vec()))
        });
}

#[test]
fn noop_good() {
    let mock = MockAntS::new(
        Ok(()),
        Ok(()),
        Ok(()),
        Ok(()),
        Ok(()),
        Ok(()),
        Ok(()),
        Ok(DeployStatus::default()),
        Ok(0),
        Ok(AntsTelemetry::default()),
        Ok(0),
        Ok(0),
        Ok(()),
        Ok(()),
        Ok(()),
        Ok(()),
    );

    mock.watchdog_kick.return_value(Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.noop().unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
}
