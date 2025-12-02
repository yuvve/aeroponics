rules.JSRule({
  name: "Tower 1 pH Level Alert",
  description: "Updates tower_1_ph_level_alert_status based on settings",
  triggers: [
    triggers.ItemStateChangeTrigger('tower_1_ph'),
    triggers.ItemStateChangeTrigger('tower_1_alerts_toggle'),
    triggers.ItemStateChangeTrigger('tower_1_low_ph_alert_setting'),
    triggers.ItemStateChangeTrigger('tower_1_high_ph_alert_setting'),
  ],
  execute: () => {
    const alerts_on = items.getItem('tower_1_alerts_toggle').state === 'ON';
    const min_ph_level = items.getItem('tower_1_low_ph_level_alert_setting').numericState;
    const max_ph_level = items.getItem('tower_1_high_ph_level_alert_setting').numericState;
    const ph_level = items.getItem('tower_1_ph').numericState;

    const alertItem = items.getItem('tower_1_ph_alert_status');

    if (alerts_on == 'ON' && (ph_level < min_ph_level || ph_level > max_ph_level)) {
      alertItem.postUpdate('ON');
    } else {
      alertItem.postUpdate('OFF');
    }
  }
});
