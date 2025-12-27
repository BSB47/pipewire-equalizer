use pw_util::config::MANAGED_PROP;
use tabled::Tabled;

#[derive(Tabled)]
pub struct EqMeta {
    id: u32,
    name: String,
}

pub async fn list_eqs() -> anyhow::Result<Vec<EqMeta>> {
    let objects = pw_util::dump().await?;

    let eqs = objects
        .into_iter()
        .filter(is_managed_eq)
        .filter(|obj| matches!(obj.object_type, pw_util::PwObjectType::Node))
        .map(|obj| {
            let props = &obj.info.props;
            let name = props
                .get("media.name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");
            EqMeta {
                id: obj.id,
                name: name.to_string(),
            }
        })
        .collect();

    Ok(eqs)
}

pub fn is_managed_eq(props: &pw_util::PwDumpObject) -> bool {
    props
        .info
        .props
        .get(MANAGED_PROP)
        .is_some_and(|managed| managed == true)
}

/// Find an EQ node by profile name or ID
pub async fn find_eq_node(profile: &str) -> anyhow::Result<pw_util::PwDumpObject> {
    let objects = pw_util::dump().await?;

    // Try to parse as ID first
    let target_id: Option<u32> = profile.parse().ok();

    objects
        .into_iter()
        .filter(|obj| matches!(obj.object_type, pw_util::PwObjectType::Node))
        .filter(is_managed_eq)
        .find(|obj| {
            if let Some(target_id) = target_id {
                obj.id == target_id
            } else {
                let props = &obj.info.props;
                if let Some(name) = props.get("media.name") {
                    name == profile
                } else {
                    false
                }
            }
        })
        .ok_or_else(|| anyhow::anyhow!("EQ '{profile}' not found"))
}

pub async fn use_eq(profile: &str) -> anyhow::Result<()> {
    // This is a placeholder implementation
    // In a real implementation, you would set the default sink to the EQ node
    println!(
        "Setting EQ '{}' as the default sink (not yet implemented)",
        profile
    );
    Ok(())
}
