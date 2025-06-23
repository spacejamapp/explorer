-- Add migration script here
CREATE TABLE IF NOT EXISTS work_results (
  id SERIAL PRIMARY KEY,
  guarantee INT NOT NULL REFERENCES guarantees(id),
  service INT NOT NULL REFERENCES services(id),
  code VARCHAR NOT NULL,
  payload VARCHAR NOT NULL,
  gas BIGINT NOT NULL,
  result VARCHAR NOT NULL,
  refine_gas BIGINT NOT NULL,
  refine_imports INT NOT NULL,
  refine_extrinsic_count INT NOT NULL,
  refine_extrinsic_size INT NOT NULL,
  refine_exports INT NOT NULL
);
CREATE INDEX idx_work_results_guarantee ON work_results (guarantee);
CREATE INDEX idx_work_results_service ON work_results (service);
