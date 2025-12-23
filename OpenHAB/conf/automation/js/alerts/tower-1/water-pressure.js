rules.JSRule({
  name: "Tower 1 Water Pressure Alert",
  description: "Updates tower_1_water_pressure_alert_status based on settings",
  triggers: [
    triggers.ItemStateChangeTrigger('tower_1_pressure'),
    triggers.ItemStateChangeTrigger('tower_1_alerts_toggle'),
    triggers.ItemStateChangeTrigger('tower_1_low_water_pressure_alert_setting'),
  ],
  execute: () => {
    const alerts_on = items.getItem('tower_1_alerts_toggle').state === 'ON';
    const min_water_pressure = items.getItem('tower_1_low_water_pressure_alert_setting').numericState;
    const water_pressure = items.getItem('tower_1_pressure').numericState;

    const alertItem = items.getItem('tower_1_water_pressure_alert_status');

    if (alerts_on && water_pressure < min_water_pressure) {
      alertItem.postUpdate('ON');
    } else {
      alertItem.postUpdate('OFF');
    }
  }
});
