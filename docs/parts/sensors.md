# What sensors do we need?
- Temperature 
- Humidity
- Nutrition (NPK sensors exist for soil, sensors for individual ions (e.g. potassium) exists for water but seem prohibitively expensive)
- Sonic sensor (for water level)
- Conductivity
- In-line pressure sensor

# Mindstorming
- If we germinate them in soil, we could maybe keep some of it when we put them in the tower and probe with NPK sensors made for soil. The problem is it's difficult to automate this, meaning the operator has to manually put the sensors in.

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
