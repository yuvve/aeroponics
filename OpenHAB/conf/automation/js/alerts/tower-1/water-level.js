rules.JSRule({
  name: "Tower 1 Water Level Alert",
  description: "Updates tower_1_water_level_alert_status based on settings",
  triggers: [
    triggers.ItemStateChangeTrigger('tower_1_water_level'),
    triggers.ItemStateChangeTrigger('tower_1_alerts_toggle'),
    triggers.ItemStateChangeTrigger('tower_1_low_water_level_alert_setting'),
  ],
  execute: () => {
    const alerts_on = items.getItem('tower_1_alerts_toggle').state === 'ON';
    const min_water_level = items.getItem('tower_1_low_water_level_alert_setting').numericState;
    const water_level = items.getItem('tower_1_water_level').numericState;

    const alertItem = items.getItem('tower_1_water_level_alert_status');

    if (alerts_on && water_level < min_water_level) {
      alertItem.postUpdate('ON');
    } else {
      alertItem.postUpdate('OFF');
    }
  }
});
