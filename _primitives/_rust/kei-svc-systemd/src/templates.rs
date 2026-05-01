// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! systemd unit + timer file templates.

use kei_runtime_core::traits::service::{RestartPolicy, ServiceUnit, TimerSpec};

pub fn render_service(u: &ServiceUnit) -> String {
    let restart = match u.restart_policy {
        RestartPolicy::Always => "always",
        RestartPolicy::OnFailure => "on-failure",
        RestartPolicy::Never => "no",
    };
    let user_line = u
        .user
        .as_deref()
        .map(|x| format!("User={x}\n"))
        .unwrap_or_default();
    let env_lines: String = u
        .env
        .iter()
        .map(|(k, v)| format!("Environment=\"{k}={v}\"\n"))
        .collect();
    let exec_args = if u.args.is_empty() {
        String::new()
    } else {
        format!(" {}", u.args.join(" "))
    };
    format!(
        "[Unit]
Description=kei-runtime managed service: {name}
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
{user_line}WorkingDirectory={wd}
ExecStart={exec}{args}
Restart={restart}
RestartSec=5
{env_lines}

[Install]
WantedBy=multi-user.target
",
        name = u.name,
        user_line = user_line,
        wd = u.working_dir,
        exec = u.exec_path,
        args = exec_args,
        restart = restart,
        env_lines = env_lines,
    )
}

pub fn render_timer(name: &str, spec: &TimerSpec) -> String {
    format!(
        "[Unit]
Description=kei-runtime timer: {name}

[Timer]
OnCalendar={oc}
RandomizedDelaySec={rds}
Persistent=true
Unit={name}.service

[Install]
WantedBy=timers.target
",
        name = name,
        oc = spec.on_calendar,
        rds = spec.randomized_delay_sec,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit() -> ServiceUnit {
        ServiceUnit {
            name: "kei-runtime".into(),
            exec_path: "/opt/keiseikit/bin/kei-runtime".into(),
            args: vec!["run".into()],
            env: vec![("KEI_MODE".into(), "managed".into())],
            working_dir: "/opt/keiseikit".into(),
            user: Some("keisei".into()),
            restart_policy: RestartPolicy::OnFailure,
            timer_spec: Some(TimerSpec { on_calendar: "*-*-* 03:07:00".into(), randomized_delay_sec: 900 }),
        }
    }

    #[test]
    fn renders_service() {
        let s = render_service(&unit());
        assert!(s.contains("ExecStart=/opt/keiseikit/bin/kei-runtime run"));
        assert!(s.contains("User=keisei"));
        assert!(s.contains("Restart=on-failure"));
        assert!(s.contains("Environment=\"KEI_MODE=managed\""));
        assert!(s.contains("WantedBy=multi-user.target"));
    }

    #[test]
    fn renders_timer() {
        let t = render_timer("kei-runtime", &TimerSpec {
            on_calendar: "*-*-* 03:07:00".into(),
            randomized_delay_sec: 900,
        });
        assert!(t.contains("OnCalendar=*-*-* 03:07:00"));
        assert!(t.contains("RandomizedDelaySec=900"));
        assert!(t.contains("Unit=kei-runtime.service"));
    }

    #[test]
    fn renders_without_user() {
        let mut u = unit();
        u.user = None;
        let s = render_service(&u);
        assert!(!s.contains("User="));
    }
}
