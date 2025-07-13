-- Your SQL goes here
CREATE TABLE IF NOT EXISTS kriteria_efek_bersifat_ekuitas_dalam_pemantauan_khusus (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    no INTEGER NOT NULL,
    keterangan TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE (no, keterangan)
);

CREATE TABLE IF NOT EXISTS daftar_efek_bersifat_ekuitas_dalam_pemantauan_khusus (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    no INTEGER NOT NULL,
    kode_saham VARCHAR(5) NOT NULL,
    nama_perusahaan VARCHAR(100) NOT NULL,
    tanggal_masuk DATE NULL,
    tanggal_keluar DATE NULL,
    kriteria VARCHAR(20) NOT NULL,
    kriteria_efek_dalam_pemantauan_khusus INTEGER NULL,
    keterangan VARCHAR(15) NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE (kode_saham)
);

CREATE TABLE IF NOT EXISTS notifikasi_khusus (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    notasi VARCHAR(1) NOT NULL,
    deskripsi TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE (notasi)
);

CREATE TABLE IF NOT EXISTS list_saham_dengan_notifikasi_khusus (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    no INTEGER NOT NULL,
    code VARCHAR(5) NOT NULL,
    name VARCHAR(100) NOT NULL,
    notasi VARCHAR(15) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE (no, code)
);