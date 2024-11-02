CREATE TABLE Microcontrollers(
       id INTEGER PRIMARY KEY AUTOINCREMENT);


CREATE TABLE Sensors(
	m_id INT NOT NULL,
    s_id INT NOT NULL,
    PRIMARY KEY (m_id, s_id),
	FOREIGN KEY (m_id) REFERENCES Microcontrollers(id) ON DELETE CASCADE);


CREATE TABLE SensorData(
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       timepoint TEXT NOT NULL,
       m_id INT NOT NULL,
       s_id INT NOT NULL,
       sensor_type INT NOT NULL,
       sensor_data INT NOT NULL,
       dummy BOOLEAN NOT NULL,       
       FOREIGN KEY (m_id, s_id) REFERENCES Sensors(m_id, s_id) ON DELETE CASCADE,       
       FOREIGN KEY (m_id) REFERENCES Microcontrollers(id) ON DELETE CASCADE);

