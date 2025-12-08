rules.JSRule({
  name: "Tower 1 temp Level Alert",
  description: "Updates tower_1_temp_level_alert_status based on settings",
  triggers: [
    triggers.ItemStateChangeTrigger('tower_1_upper_temp'),
    triggers.ItemStateChangeTrigger('tower_1_alerts_toggle'),
    triggers.ItemStateChangeTrigger('tower_1_low_temp_alert_setting'),
    triggers.ItemStateChangeTrigger('tower_1_high_temp_alert_setting'),
  ],
  execute: () => {
    const alerts_on = items.getItem('tower_1_alerts_toggle').state === 'ON';
    const min_temp_level = items.getItem('tower_1_low_temp_alert_setting').numericState;
    const max_temp_level = items.getItem('tower_1_high_temp_alert_setting').numericState;
    const temp_level = items.getItem('tower_1_upper_temp').numericState;

    const alertItem = items.getItem('tower_1_temp_alert_status');

    if (alerts_on && (temp_level < min_temp_level || temp_level > max_temp_level)) {
      alertItem.postUpdate('ON');
    } else {
      alertItem.postUpdate('OFF');
    }
  }
});
