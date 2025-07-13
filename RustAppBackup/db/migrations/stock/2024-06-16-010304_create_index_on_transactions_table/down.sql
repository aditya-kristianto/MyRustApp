-- This file should undo anything in `up.sql`

-- Drop index for the column 'tanggal_perdagangan_terakhir'
DROP INDEX IF EXISTS idx_transactions_tanggal_perdagangan_terakhir;

-- Drop index for the column 'kode_saham'
DROP INDEX IF EXISTS idx_transactions_kode_saham;

-- Drop composite index for 'kode_saham' and 'tanggal_perdagangan_terakhir'
DROP INDEX IF EXISTS idx_transactions_kode_saham_tanggal;

-- Drop composite index for 'kode_saham' and 'penutupan'
DROP INDEX IF EXISTS idx_transactions_kode_saham_penutupan;

-- Drop composite index for 'kode_saham', 'penutupan' and 'tanggal_perdagangan_terakhir'
DROP INDEX IF EXISTS idx_transactions_kode_saham_penutupan_tanggal;
