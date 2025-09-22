# Purchasing list (first batch)
- 4x PT100 https://www.electrokit.com/pt-100-givare (LM75 as digital alternative) or https://se.farnell.com/labfacility/dm-508/sensor-pt100-thin-film-2x5mm-cl/dp/1289666
- SEN0244 TDS sensor https://eu.mouser.com/ProductDetail/DFRobot/SEN0244?qs=W0yvOO0ixfHyUmHuEPwC1w%3D%3D or https://se.farnell.com/dfrobot/sen0244/analogue-tds-sensor-meter-kit/dp/3517934
- Sonic sensor x4 (for sealant testing) https://www.electrokit.com/avstandsmatare-ultraljud-hc-sr04-2-400cm or https://se.farnell.com/multicomp-pro/hc-sr04/ultrasonic-sensor-40khz-4-5m/dp/4162009
- pH sensor https://www.mouser.se/ProductDetail/DFRobot/SEN0169?qs=lqAf%2FiVYw9h7p9WXQvmXEQ%3D%3D or https://se.farnell.com/dfrobot/sen0169-v2/ph-sensor-meter-pro-kit-v2-arduino/dp/3769984
- Potentiometer https://se.farnell.com/vishay/t93yb201kt20/trimmer-pot-200r-23turn-thd/dp/1141413 (one or two at e.g. 50 Ohm, 100 Ohm and 200 Ohm)
- Humidity sensor HS30P x4 https://se.farnell.com/amphenol-advanced-sensors/hs30p/sensor-output-type/dp/3565570
- Total cost:  ≈ 2000 SEK

# What sensors do we need?
- Temperature 
- Humidity
- Nutrition (~~NPK sensors exist for soil, sensors for individual ions (e.g. potassium) exists for water but seem prohibitively expensive~~ Could not find accessible NPK sensor for purchase)
- Sonic sensor (for water level)
- Conductivity
- In-line pressure sensor

# Mindstorming
- ~~If we germinate them in soil, we could maybe keep some of it when we put them in the tower and probe with NPK sensors made for soil. The problem is it's difficult to automate this, meaning the operator has to manually put the sensors in.~~

# Requirements
- Has to work in high humidity or in water
- Has to fit parts (e.g. in-line pressure sensor needs to fit into the high pressure hoses we choose)

# Specific sensors
- Nutrition
	- JXCT JXBS-3001-NPK-RS (detects content of nitrogen, phosphorus and potassium in soil. Can it be used in water? Datasheet mentions operable humidity interval)
- Temperature
	- TMP36 TO-92 (-40 to 125 degrees C)
- Conductivity
	- TDS sensors might have potential, e.g. SEN0244
	
	
	
# Links
- Datasheets
	- https://www.manualslib.com/manual/2843998/Jxct-Jxbs-3001-Npk-Rs.html?page=3#manual
	- https://www.electrokit.com/tmp36-to-92-temperatursensor-40...125c
	- https://mm.digikey.com/Volume0/opasdata/d220001/medias/docus/2309/SEN0244_Web.pdf
