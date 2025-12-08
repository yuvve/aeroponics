rules.JSRule({
  name: "Tower 1 ec Level Alert",
  description: "Updates tower_1_ec_alert_status based on settings",
  triggers: [
    triggers.ItemStateChangeTrigger('tower_1_ec'),
    triggers.ItemStateChangeTrigger('tower_1_alerts_toggle'),
    triggers.ItemStateChangeTrigger('tower_1_low_ec_alert_setting'),
    triggers.ItemStateChangeTrigger('tower_1_high_ec_alert_setting'),
  ],
  execute: () => {
    const alerts_on = items.getItem('tower_1_alerts_toggle').state === 'ON';
    const min_ec_level = items.getItem('tower_1_low_ec_alert_setting').numericState;
    const max_ec_level = items.getItem('tower_1_high_ec_alert_setting').numericState;
    const ec_level = items.getItem('tower_1_ec').numericState;

    const alertItem = items.getItem('tower_1_ec_alert_status');

    if (alerts_on && (ec_level < min_ec_level || ec_level > max_ec_level)) {
      alertItem.postUpdate('ON');
    } else {
      alertItem.postUpdate('OFF');
    }
  }
});
