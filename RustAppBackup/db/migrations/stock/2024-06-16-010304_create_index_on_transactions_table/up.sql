-- Your SQL goes here

-- Index for the column 'tanggal_perdagangan_terakhir'
CREATE INDEX idx_transactions_tanggal_perdagangan_terakhir
ON transactions(tanggal_perdagangan_terakhir);

-- Index for the column 'kode_saham'
CREATE INDEX idx_transactions_kode_saham
ON transactions(kode_saham);

-- Composite index for 'kode_saham' and 'tanggal_perdagangan_terakhir'
CREATE INDEX idx_transactions_kode_saham_tanggal
ON transactions(kode_saham, tanggal_perdagangan_terakhir);

-- Composite index for 'kode_saham' and 'penutupan'
CREATE INDEX idx_transactions_kode_saham_penutupan
ON transactions(kode_saham, penutupan);

-- Composite index for 'kode_saham', 'penutupan' and 'tanggal_perdagangan_terakhir'
CREATE INDEX idx_transactions_kode_saham_penutupan_tanggal
ON transactions(kode_saham, penutupan, tanggal_perdagangan_terakhir);
