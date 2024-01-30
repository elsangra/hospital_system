use std::collections::HashMap;

#[derive(Debug, Clone)]
struct PatientData {
    patient_id: u64,
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u64,
    identification_no: u64,
    birth_cert_no: u64,
    passport_no: u64,
    gender: String,
    blood_group: String,
    medical_report: String,
}

#[derive(Debug)]
struct PatientRegistry {
    patients: HashMap<u64, PatientData>,
}

impl PatientRegistry {
    fn new() -> Self {
        Self {
            patients: HashMap::new(),
        }
    }

    fn upload_patient_data(&mut self, patient_data: PatientData) {
        self.patients.insert(patient_data.patient_id, patient_data);
    }

    fn read_patient_data(&self, id: u64) -> Option<&PatientData> {
        self.patients.get(&id)
    }

    fn update_patient_data(&mut self, patient_data: PatientData, id: u64) -> String {
        if let Some(existing_patient) = self.patients.get_mut(&id) {
            // Update the fields as needed
            existing_patient.first_name = patient_data.first_name;
            existing_patient.middle_name = patient_data.middle_name;
            existing_patient.last_name = patient_data.last_name;
            existing_patient.age = patient_data.age;
            existing_patient.identification_no = patient_data.identification_no;
            existing_patient.birth_cert_no = patient_data.birth_cert_no;
            existing_patient.passport_no = patient_data.passport_no;
            existing_patient.gender = patient_data.gender;
            existing_patient.blood_group = patient_data.blood_group;
            existing_patient.medical_report = patient_data.medical_report;

            "Updated successfully".to_string()
        } else {
            "Such patient does not exist".to_string()
        }
    }
}

fn main() {
    let mut patient_registry = PatientRegistry::new();

    // Example data
    let patient_data = PatientData {
        patient_id: 1,
        first_name: String::from("John"),
        middle_name: String::from("Doe"),
        last_name: String::from("Smith"),
        age: 30,
        identification_no: 12345,
        birth_cert_no: 67890,
        passport_no: 54321,
        gender: String::from("Male"),
        blood_group: String::from("O+"),
        medical_report: String::from("Some medical report"),
    };

    // Upload patient data
    patient_registry.upload_patient_data(patient_data.clone());

    // Read patient data
    match patient_registry.read_patient_data(1) {
        Some(patient) => println!("Read patient data: {:?}", patient),
        None => println!("Patient not found"),
    }

    // Update patient data
    let updated_data = PatientData {
        first_name: String::from("Updated John"),
        // ... other updated fields
        ..patient_data.clone()
    };
    let result = patient_registry.update_patient_data(updated_data, 1);
    println!("{}", result);
}
