% SD Card Party
% DMA Transfers, Dateisystemzugriff und SD Initialisierung
% Christoffer Anselm, Fabian Hinderer, Clara Scherer



# Showcase

# SD Initialisierung

## SdHandle

[//]: ![](SdHandle.png)\


## CardInfo

[//]: ![](CardInfo.png)\


## Hardware initialisieren

- GPIO Pins per Alternate Function setzen
	- SDMMC Clock
	- SDMMC Command
	- SDMMC Data

- Clock initialisieren und anschalten

- Power On

- Initialisierung der SD Karte

## SD Karte initialisieren

[//]: ![](Initialization.png)\


## Benutzung

- DmaManager initialisieren $\rightarrow$ dma::DmaManager::init_dma2(dma_2, rcc);
- neues SdHandle struct erzeugen $\rightarrow$ sd::SdHandle::new(sdmmc, &dma_2, &mut sdram_addr);
- SdHandle initialisieren $\rightarrow$ sd_handle.init(&mut gpio, rcc);

[//]: ![](Benutzung.png)\

