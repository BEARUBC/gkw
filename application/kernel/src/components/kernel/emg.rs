use anyhow::Result;

#[derive(Default)]
pub(super) struct State;

#[cfg(feature = "pseudo_analytics")]
pub(super) fn parser(_: &mut State, _: f64) -> Result<()> {
    let next_grip = GripType::from(data);

    if(state.current_grip == next_grip) {
        return Ok(());
    }

    match next_grip {
        GripType::Cup => {
            let channel = Channels::C_0;
            
        },

        GripType::Hammer => {

        },

        GripType::Flat => {

        }
    }
    


    


    Ok(())
}

#[cfg(not(feature = "pseudo_analytics"))]
pub(super) fn parser(_: &mut State, _: f64) -> Result<()> {
    todo!()
}
