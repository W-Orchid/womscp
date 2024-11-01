CREATE TABLE Microcontrollers(
       id INTEGER PRIMARY KEY AUTOINCREMENT);


CREATE TABLE SensorTypes(
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	sensor_type TEXT NOT NULL UNIQUE);


CREATE TABLE Sensors(
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	sensor_type INT NOT NULL,
	s_id INT NOT NULL,
	m_id INT NOT NULL,
	FOREIGN KEY (sensor_type) REFERENCES SensorTypes(id) ON DELETE CASCADE,	
	FOREIGN KEY (m_id) REFERENCES Microcontrollers(id) ON DELETE CASCADE);


CREATE TABLE SensorData(
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       timepoint TEXT NOT NULL,
       s_id INT NOT NULL,
       m_id INT NOT NULL,
       sensor_data INT NOT NULL,
       dummy BOOLEAN NOT NULL,       
       FOREIGN KEY (s_id) REFERENCES Sensors(s_id) ON DELETE CASCADE,       
       FOREIGN KEY (m_id) REFERENCES Microcontrollers(id) ON DELETE CASCADE);
