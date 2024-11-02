CREATE TABLE Microcontrollers(
       id INTEGER PRIMARY KEY AUTOINCREMENT);


CREATE TABLE Sensors(
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	s_id INT NOT NULL,
	m_id INT NOT NULL,
	FOREIGN KEY (m_id) REFERENCES Microcontrollers(id) ON DELETE CASCADE);


CREATE TABLE SensorData(
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       timepoint TEXT NOT NULL,
       s_id INT NOT NULL,
       m_id INT NOT NULL,
       sensor_data INT NOT NULL,
       sensor_type INT NOT NULL,
       dummy BOOLEAN NOT NULL,       
       FOREIGN KEY (s_id) REFERENCES Sensors(s_id) ON DELETE CASCADE,       
       FOREIGN KEY (m_id) REFERENCES Microcontrollers(id) ON DELETE CASCADE);

