#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MerchantCategories {
    AcRefrigerationRepair,                              //A/C, Refrigeration Repair
    AccountingBookkeepingServices,                      //Accounting/Bookkeeping Services
    AdvertisingServices,                                //Advertising Services
    AgriculturalCooperative,                            //Agricultural Cooperative
    AirlinesAirCarriers,                                //Airlines, Air Carriers
    AirportsFlyingFields,                               //Airports, Flying Fields
    AmbulanceServices,                                  //Ambulance Services
    AmusementParksCarnivals,                            //Amusement Parks/Carnivals
    AntiqueReproductions,                               //Antique Reproductions
    AntiqueShops,                                       //Antique Shops
    Aquariums,                                          //Aquariums
    ArchitecturalSurveyingServices,                     //Architectural/Surveying Services
    ArtDealersAndGalleries,                             //Art Dealers and Galleries
    ArtistsSupplyAndCraftShops,                         //Artists Supply and Craft Shops
    AutoBodyRepairShops,                                //Auto Body Repair Shops
    AutoPaintShops,                                     //Auto Paint Shops
    AutoServiceShops,                                   //Auto Service Shops
    AutoAndHomeSupplyStores,                            //Auto and Home Supply Stores
    AutomatedCashDisburse,                              //Automated Cash Disburse
    AutomatedFuelDispensers,                            //Automated Fuel Dispensers
    AutomobileAssociations,                             //Automobile Associations
    AutomotivePartsAndAccessoriesStores,                //Automotive Parts and Accessories Stores
    AutomotiveTireStores,                               //Automotive Tire Stores
    BailAndBondPayments, //Bail and Bond Payments (payment to the surety for the bond, not the actual bond paid to the government agency)
    Bakeries,            //Bakeries
    BandsOrchestras,     //Bands, Orchestras
    BarberAndBeautyShops, //Barber and Beauty Shops
    BettingCasinoGambling, //Betting/Casino Gambling
    BicycleShops,        //Bicycle Shops
    BilliardPoolEstablishments, //Billiard/Pool Establishments
    BoatDealers,         //Boat Dealers
    BoatRentalsAndLeases, //Boat Rentals and Leases
    BookStores,          //Book Stores
    BooksPeriodicalsAndNewspapers, //Books, Periodicals, and Newspapers
    BowlingAlleys,       //Bowling Alleys
    BusLines,            //Bus Lines
    BusinessSecretarialSchools, //Business/Secretarial Schools
    BuyingShoppingServices, //Buying/Shopping Services
    CableSatelliteAndOtherPayTelevisionAndRadio, //Cable, Satellite, and Other Pay Television and Radio
    CameraAndPhotographicSupplyStores,           //Camera and Photographic Supply Stores
    CandyNutAndConfectioneryStores,              //Candy, Nut, and Confectionery Stores
    CarRentalAgencies,                           //Car Rental Agencies
    CarWashes,                                   //Car Washes
    CarAndTruckDealersNewUsed, //Car and Truck Dealers (New & Used) Sales, Service, Repairs Parts and Leasing
    CarAndTruckDealersUsedOnly, //Car and Truck Dealers (Used Only) Sales, Service, Repairs Parts and Leasing
    CarpentryServices,          //Carpentry Services
    CarpetUpholsteryCleaning,   //Carpet/Upholstery Cleaning
    Caterers,                   //Caterers
    CharitableAndSocialServiceOrganizationsFundraising, //Charitable and Social Service Organizations - Fundraising
    ChemicalsAndAlliedProducts, //Chemicals and Allied Products (Not Elsewhere Classified)
    ChidrensAndInfantsWearStores, //Chidrens and Infants Wear Stores
    ChildCareServices,          //Child Care Services
    ChiropodistsPodiatrists,    //Chiropodists, Podiatrists
    Chiropractors,              //Chiropractors
    CigarStoresAndStands,       //Cigar Stores and Stands
    CivicSocialFraternalAssociations, //Civic, Social, Fraternal Associations
    CleaningAndMaintenance,     //Cleaning and Maintenance
    ClothingRental,             //Clothing Rental
    CollegesUniversities,       //Colleges, Universities
    CommercialEquipment,        //Commercial Equipment (Not Elsewhere Classified)
    CommercialFootwear,         //Commercial Footwear
    CommercialPhotographyArtAndGraphics, //Commercial Photography, Art and Graphics
    CommuterTransportAndFerries, //Commuter Transport, Ferries
    ComputerNetworkServices,    //Computer Network Services
    ComputerProgramming,        //Computer Programming
    ComputerRepair,             //Computer Repair
    ComputerSoftwareStores,     //Computer Software Stores
    ComputersPeripheralsAndSoftware, //Computers, Peripherals, and Software
    ConcreteWorkServices,       //Concrete Work Services
    ConstructionMaterials,      //Construction Materials (Not Elsewhere Classified)
    ConsultingPublicRelations,  //Consulting, Public Relations
    CorrespondenceSchools,      //Correspondence Schools
    CosmeticStores,             //Cosmetic Stores
    CounselingServices,         //Counseling Services
    CountryClubs,               //Country Clubs
    CourierServices,            //Courier Services
    CourtCosts,                 //Court Costs, Including Alimony and Child Support - Courts of Law
    CreditReportingAgencies,    //Credit Reporting Agencies
    CruiseLines,                //Cruise Lines
    DairyProductsStores,        //Dairy Products Stores
    DanceHallStudiosSchools,    //Dance Hall, Studios, Schools
    DatingEscortServices,       //Dating/Escort Services
    DentistsOrthodontists,      //Dentists, Orthodontists
    DepartmentStores,           //Department Stores
    DetectiveAgencies,          //Detective Agencies
    DirectMarketingCatalogMerchant, //Direct Marketing - Catalog Merchant
    DirectMarketingCombinationCatalogAndRetailMerchant, //Direct Marketing - Combination Catalog and Retail Merchant
    DirectMarketingInboundTelemarketing,                //Direct Marketing - Inbound Telemarketing
    DirectMarketingInsuranceServices,                   //Direct Marketing - Insurance Services
    DirectMarketingOther,                               //Direct Marketing - Other
    DirectMarketingOutboundTelemarketing,               //Direct Marketing - Outbound Telemarketing
    DirectMarketingSubscription,                        //Direct Marketing - Subscription
    DirectMarketingTravel,                              //Direct Marketing - Travel
    DiscountStores,                                     //Discount Stores
    Doctors,                                            //Doctors
    DoorToDoorSales,                                    //Door-To-Door Sales
    DraperyWindowCoveringAndUpholsteryStores, //Drapery, Window Covering, and Upholstery Stores
    DrinkingPlaces,                           //Drinking Places
    DrugStoresAndPharmacies,                  //Drug Stores and Pharmacies
    DrugsDrugProprietariesAndDruggistSundries, //Drugs, Drug Proprietaries, and Druggist Sundries
    DryCleaners,                              //Dry Cleaners
    DurableGoods,                             //Durable Goods (Not Elsewhere Classified)
    DutyFreeStores,                           //Duty Free Stores
    EatingPlacesRestaurants,                  //Eating Places, Restaurants
    EducationalServices,                      //Educational Services
    ElectricRazorStores,                      //Electric Razor Stores
    ElectricalPartsAndEquipment,              //Electrical Parts and Equipment
    ElectricalServices,                       //Electrical Services
    ElectronicsRepairShops,                   //Electronics Repair Shops
    ElectronicsStores,                        //Electronics Stores
    ElementarySecondarySchools,               //Elementary, Secondary Schools
    EmploymentTempAgencies,                   //Employment/Temp Agencies
    EquipmentRental,                          //Equipment Rental
    ExterminatingServices,                    //Exterminating Services
    FamilyClothingStores,                     //Family Clothing Stores
    FastFoodRestaurants,                      //Fast Food Restaurants
    FinancialInstitutions,                    //Financial Institutions
    FinesGovernmentAdministrativeEntities,    //Fines - Government Administrative Entities
    FireplaceFireplaceScreensAndAccessoriesStores, //Fireplace, Fireplace Screens, and Accessories Stores
    FloorCoveringStores,                           //Floor Covering Stores
    Florists,                                      //Florists
    FloristsSuppliesNurseryStockAndFlowers,        //Florists Supplies, Nursery Stock, and Flowers
    FreezerAndLockerMeatProvisioners,              //Freezer and Locker Meat Provisioners
    FuelDealersNonAutomotive,                      //Fuel Dealers (Non Automotive)
    FuneralServicesCrematories,                    //Funeral Services, Crematories
    FurnitureRepairRefinishing,                    //Furniture Repair, Refinishing
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances, //Furniture, Home Furnishings, and Equipment Stores, Except Appliances
    FurriersAndFurShops,                                        //Furriers and Fur Shops
    GeneralServices,                                            //General Services
    GiftCardNoveltyAndSouvenirShops, //Gift, Card, Novelty, and Souvenir Shops
    GlassPaintAndWallpaperStores,    //Glass, Paint, and Wallpaper Stores
    GlasswareCrystalStores,          //Glassware, Crystal Stores
    GolfCoursesPublic,               //Golf Courses - Public
    GovernmentServices,              //Government Services (Not Elsewhere Classified)
    GroceryStoresSupermarkets,       //Grocery Stores, Supermarkets
    HardwareStores,                  //Hardware Stores
    HardwareEquipmentAndSupplies,    //Hardware, Equipment, and Supplies
    HealthAndBeautySpas,             //Health and Beauty Spas
    HearingAidsSalesAndSupplies,     //Hearing Aids Sales and Supplies
    HeatingPlumbingAC,               //Heating, Plumbing, A/C
    HobbyToyAndGameShops,            //Hobby, Toy, and Game Shops
    HomeSupplyWarehouseStores,       //Home Supply Warehouse Stores
    Hospitals,                       //Hospitals
    HotelsMotelsAndResorts,          //Hotels, Motels, and Resorts
    HouseholdApplianceStores,        //Household Appliance Stores
    IndustrialSupplies,              //Industrial Supplies (Not Elsewhere Classified)
    InformationRetrievalServices,    //Information Retrieval Services
    InsuranceDefault,                //Insurance - Default
    InsuranceUnderwritingPremiums,   //Insurance Underwriting, Premiums
    IntraCompanyPurchases,           //Intra-Company Purchases
    JewelryStoresWatchesClocksAndSilverwareStores, //Jewelry Stores, Watches, Clocks, and Silverware Stores
    LandscapingServices,                           //Landscaping Services
    Laundries,                                     //Laundries
    LaundryCleaningServices,                       //Laundry, Cleaning Services
    LegalServicesAttorneys,                        //Legal Services, Attorneys
    LuggageAndLeatherGoodsStores,                  //Luggage and Leather Goods Stores
    LumberBuildingMaterialsStores,                 //Lumber, Building Materials Stores
    ManualCashDisburse,                            //Manual Cash Disburse
    MarinasServiceAndSupplies,                     //Marinas, Service and Supplies
    MasonryStoneworkAndPlaster,                    //Masonry, Stonework, and Plaster
    MassageParlors,                                //Massage Parlors
    MeansWomensClothingStores,                     //Means, Womens Clothing Stores
    MedicalServices,                               //Medical Services
    MedicalAndDentalLabs,                          //Medical and Dental Labs
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies, //Medical, Dental, Ophthalmic, and Hospital Equipment and Supplies
    MembershipOrganizations,                                //Membership Organizations
    MensAndBoysClothingAndAccessoriesStores, //Mens and Boys Clothing and Accessories Stores
    MetalServiceCenters,                     //Metal Service Centers
    MiscellaneousApparelAndAccessoryShops,   //Miscellaneous Apparel and Accessory Shops
    MiscellaneousAutoDealers,                //Miscellaneous Auto Dealers
    MiscellaneousBusinessServices,           //Miscellaneous Business Services
    MiscellaneousFoodStores, //Miscellaneous Food Stores - Convenience Stores and Specialty Markets
    MiscellaneousGeneralMerchandise, //Miscellaneous General Merchandise
    MiscellaneousGeneralServices, //Miscellaneous General Services
    MiscellaneousHomeFurnishingSpecialtyStores, //Miscellaneous Home Furnishing Specialty Stores
    MiscellaneousPublishingAndPrinting, //Miscellaneous Publishing and Printing
    MiscellaneousRecreationServices, //Miscellaneous Recreation Services
    MiscellaneousRepairShops, //Miscellaneous Repair Shops
    MiscellaneousSpecialtyRetail, //Miscellaneous Specialty Retail
    MobileHomeDealers,       //Mobile Home Dealers
    MotionPictureTheaters,   //Motion Picture Theaters
    MotorFreightCarriersAndTrucking, //Motor Freight Carriers and Trucking - Local and Long Distance, Moving and Storage Companies, and Local Delivery Services
    MotorHomesDealers,               //Motor Homes Dealers
    MotorVehicleSuppliesAndNewParts, //Motor Vehicle Supplies and New Parts
    MotorcycleShopsAndDealers,       //Motorcycle Shops and Dealers
    MotorcycleShopsDealers,          //Motorcycle Shops, Dealers
    MusicStoresMusicalInstrumentsPianosAndSheetMusic, //Music Stores-Musical Instruments, Pianos, and Sheet Music
    NewsDealersAndNewsstands,                         //News Dealers and Newsstands
    NonFiMoneyOrders,                                 //Non-FI, Money Orders
    NonFiStoredValueCardPurchaseLoad,                 //Non-FI, Stored Value Card Purchase/Load
    NondurableGoods,                                  //Nondurable Goods (Not Elsewhere Classified)
    NurseriesLawnAndGardenSupplyStores,               //Nurseries, Lawn and Garden Supply Stores
    NursingPersonalCare,                              //Nursing/Personal Care
    OfficeAndCommercialFurniture,                     //Office and Commercial Furniture
    OpticiansEyeglasses,                              //Opticians, Eyeglasses
    OptometristsOphthalmologist,                      //Optometrists, Ophthalmologist
    OrthopedicGoodsProstheticDevices,                 //Orthopedic Goods - Prosthetic Devices
    Osteopaths,                                       //Osteopaths
    PackageStoresBeerWineAndLiquor,                   //Package Stores-Beer, Wine, and Liquor
    PaintsVarnishesAndSupplies,                       //Paints, Varnishes, and Supplies
    ParkingLotsGarages,                               //Parking Lots, Garages
    PassengerRailways,                                //Passenger Railways
    PawnShops,                                        //Pawn Shops
    PetShopsPetFoodAndSupplies,                       //Pet Shops, Pet Food, and Supplies
    PetroleumAndPetroleumProducts,                    //Petroleum and Petroleum Products
    PhotoDeveloping,                                  //Photo Developing
    PhotographicStudios,                              //Photographic Studios
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies, //Photographic, Photocopy, Microfilm Equipment, and Supplies
    PictureVideoProduction,                             //Picture/Video Production
    PieceGoodsNotionsAndOtherDryGoods,                  //Piece Goods, Notions, and Other Dry Goods
    PlumbingHeatingEquipmentAndSupplies,                //Plumbing, Heating Equipment, and Supplies
    PoliticalOrganizations,                             //Political Organizations
    PostalServicesGovernmentOnly,                       //Postal Services - Government Only
    PreciousStonesAndMetalsWatchesAndJewelry, //Precious Stones and Metals, Watches and Jewelry
    ProfessionalServices,                     //Professional Services
    PublicWarehousingAndStorage, //Public Warehousing and Storage - Farm Products, Refrigerated Goods, Household Goods, and Storage
    QuickCopyReproAndBlueprint,  //Quick Copy, Repro, and Blueprint
    Railroads,                   //Railroads
    RealEstateAgentsAndManagersRentals, //Real Estate Agents and Managers - Rentals
    RecordStores,                //Record Stores
    RecreationalVehicleRentals,  //Recreational Vehicle Rentals
    ReligiousGoodsStores,        //Religious Goods Stores
    ReligiousOrganizations,      //Religious Organizations
    RoofingSidingSheetMetal,     //Roofing/Siding, Sheet Metal
    SecretarialSupportServices,  //Secretarial Support Services
    SecurityBrokersDealers,      //Security Brokers/Dealers
    ServiceStations,             //Service Stations
    SewingNeedleworkFabricAndPieceGoodsStores, //Sewing, Needlework, Fabric, and Piece Goods Stores
    ShoeRepairHatCleaning,       //Shoe Repair/Hat Cleaning
    ShoeStores,                  //Shoe Stores
    SmallApplianceRepair,        //Small Appliance Repair
    SnowmobileDealers,           //Snowmobile Dealers
    SpecialTradeServices,        //Special Trade Services
    SpecialtyCleaning,           //Specialty Cleaning
    SportingGoodsStores,         //Sporting Goods Stores
    SportingRecreationCamps,     //Sporting/Recreation Camps
    SportsClubsFields,           //Sports Clubs/Fields
    SportsAndRidingApparelStores, //Sports and Riding Apparel Stores
    StampAndCoinStores,          //Stamp and Coin Stores
    StationaryOfficeSuppliesPrintingAndWritingPaper, //Stationary, Office Supplies, Printing and Writing Paper
    StationeryStoresOfficeAndSchoolSupplyStores, //Stationery Stores, Office, and School Supply Stores
    SwimmingPoolsSales,                          //Swimming Pools Sales
    TUiTravelGermany,                            //TUI Travel - Germany
    TailorsAlterations,                          //Tailors, Alterations
    TaxPaymentsGovernmentAgencies,               //Tax Payments - Government Agencies
    TaxPreparationServices,                      //Tax Preparation Services
    TaxicabsLimousines,                          //Taxicabs/Limousines
    TelecommunicationEquipmentAndTelephoneSales, //Telecommunication Equipment and Telephone Sales
    TelecommunicationServices,                   //Telecommunication Services
    TelegraphServices,                           //Telegraph Services
    TentAndAwningShops,                          //Tent and Awning Shops
    TestingLaboratories,                         //Testing Laboratories
    TheatricalTicketAgencies,                    //Theatrical Ticket Agencies
    Timeshares,                                  //Timeshares
    TireRetreadingAndRepair,                     //Tire Retreading and Repair
    TollsBridgeFees,                             //Tolls/Bridge Fees
    TouristAttractionsAndExhibits,               //Tourist Attractions and Exhibits
    TowingServices,                              //Towing Services
    TrailerParksCampgrounds,                     //Trailer Parks, Campgrounds
    TransportationServices, //Transportation Services (Not Elsewhere Classified)
    TravelAgenciesTourOperators, //Travel Agencies, Tour Operators
    TruckStopIteration,     //Truck StopIteration
    TruckUtilityTrailerRentals, //Truck/Utility Trailer Rentals
    TypesettingPlateMakingAndRelatedServices, //Typesetting, Plate Making, and Related Services
    TypewriterStores,       //Typewriter Stores
    USFederalGovernmentAgenciesOrDepartments, //U.S. Federal Government Agencies or Departments
    UniformsCommercialClothing, //Uniforms, Commercial Clothing
    UsedMerchandiseAndSecondhandStores, //Used Merchandise and Secondhand Stores
    Utilities,              //Utilities
    VarietyStores,          //Variety Stores
    VeterinaryServices,     //Veterinary Services
    VideoAmusementGameSupplies, //Video Amusement Game Supplies
    VideoGameArcades,       //Video Game Arcades
    VideoTapeRentalStores,  //Video Tape Rental Stores
    VocationalTradeSchools, //Vocational/Trade Schools
    WatchJewelryRepair,     //Watch/Jewelry Repair
    WeldingRepair,          //Welding Repair
    WholesaleClubs,         //Wholesale Clubs
    WigAndToupeeStores,     //Wig and Toupee Stores
    WiresMoneyOrders,       //Wires, Money Orders
    WomensAccessoryAndSpecialtyShops, //Womens Accessory and Specialty Shops
    WomensReadyToWearStores, //Womens Ready-To-Wear Stores
    WreckingAndSalvageYards, //Wrecking and Salvage Yards
    #[serde(other, skip_serializing)]
    Unknown,
}

impl MerchantCategories {
    pub fn to_mcc(&self) -> i32 {
        match *self {
            MerchantCategories::AcRefrigerationRepair => 7623,
            MerchantCategories::AccountingBookkeepingServices => 8931,
            MerchantCategories::AdvertisingServices => 7311,
            MerchantCategories::AgriculturalCooperative => 763,
            MerchantCategories::AirlinesAirCarriers => 4511,
            MerchantCategories::AirportsFlyingFields => 4582,
            MerchantCategories::AmbulanceServices => 4119,
            MerchantCategories::AmusementParksCarnivals => 7996,
            MerchantCategories::AntiqueReproductions => 5937,
            MerchantCategories::AntiqueShops => 5932,
            MerchantCategories::Aquariums => 7998,
            MerchantCategories::ArchitecturalSurveyingServices => 8911,
            MerchantCategories::ArtDealersAndGalleries => 5971,
            MerchantCategories::ArtistsSupplyAndCraftShops => 5970,
            MerchantCategories::AutoBodyRepairShops => 7531,
            MerchantCategories::AutoPaintShops => 7535,
            MerchantCategories::AutoServiceShops => 7538,
            MerchantCategories::AutoAndHomeSupplyStores => 5531,
            MerchantCategories::AutomatedCashDisburse => 6011,
            MerchantCategories::AutomatedFuelDispensers => 5542,
            MerchantCategories::AutomobileAssociations => 8675,
            MerchantCategories::AutomotivePartsAndAccessoriesStores => 5533,
            MerchantCategories::AutomotiveTireStores => 5532,
            MerchantCategories::BailAndBondPayments => 9223,
            MerchantCategories::Bakeries => 5462,
            MerchantCategories::BandsOrchestras => 7929,
            MerchantCategories::BarberAndBeautyShops => 7230,
            MerchantCategories::BettingCasinoGambling => 7995,
            MerchantCategories::BicycleShops => 5940,
            MerchantCategories::BilliardPoolEstablishments => 7932,
            MerchantCategories::BoatDealers => 5551,
            MerchantCategories::BoatRentalsAndLeases => 4457,
            MerchantCategories::BookStores => 5942,
            MerchantCategories::BooksPeriodicalsAndNewspapers => 5192,
            MerchantCategories::BowlingAlleys => 7933,
            MerchantCategories::BusLines => 4131,
            MerchantCategories::BusinessSecretarialSchools => 8244,
            MerchantCategories::BuyingShoppingServices => 7278,
            MerchantCategories::CableSatelliteAndOtherPayTelevisionAndRadio => 4899,
            MerchantCategories::CameraAndPhotographicSupplyStores => 5946,
            MerchantCategories::CandyNutAndConfectioneryStores => 5441,
            MerchantCategories::CarRentalAgencies => 7512,
            MerchantCategories::CarWashes => 7542,
            MerchantCategories::CarAndTruckDealersNewUsed => 5511,
            MerchantCategories::CarAndTruckDealersUsedOnly => 5521,
            MerchantCategories::CarpentryServices => 1750,
            MerchantCategories::CarpetUpholsteryCleaning => 7217,
            MerchantCategories::Caterers => 5811,
            MerchantCategories::CharitableAndSocialServiceOrganizationsFundraising => 8398,
            MerchantCategories::ChemicalsAndAlliedProducts => 5169,
            MerchantCategories::ChidrensAndInfantsWearStores => 5641,
            MerchantCategories::ChildCareServices => 8351,
            MerchantCategories::ChiropodistsPodiatrists => 8049,
            MerchantCategories::Chiropractors => 8041,
            MerchantCategories::CigarStoresAndStands => 5993,
            MerchantCategories::CivicSocialFraternalAssociations => 8641,
            MerchantCategories::CleaningAndMaintenance => 7349,
            MerchantCategories::ClothingRental => 7296,
            MerchantCategories::CollegesUniversities => 8220,
            MerchantCategories::CommercialEquipment => 5046,
            MerchantCategories::CommercialFootwear => 5139,
            MerchantCategories::CommercialPhotographyArtAndGraphics => 7333,
            MerchantCategories::CommuterTransportAndFerries => 4111,
            MerchantCategories::ComputerNetworkServices => 4816,
            MerchantCategories::ComputerProgramming => 7372,
            MerchantCategories::ComputerRepair => 7379,
            MerchantCategories::ComputerSoftwareStores => 5734,
            MerchantCategories::ComputersPeripheralsAndSoftware => 5045,
            MerchantCategories::ConcreteWorkServices => 1771,
            MerchantCategories::ConstructionMaterials => 5039,
            MerchantCategories::ConsultingPublicRelations => 7392,
            MerchantCategories::CorrespondenceSchools => 8241,
            MerchantCategories::CosmeticStores => 5977,
            MerchantCategories::CounselingServices => 7277,
            MerchantCategories::CountryClubs => 7997,
            MerchantCategories::CourierServices => 4215,
            MerchantCategories::CourtCosts => 9211,
            MerchantCategories::CreditReportingAgencies => 7321,
            MerchantCategories::CruiseLines => 4411,
            MerchantCategories::DairyProductsStores => 5451,
            MerchantCategories::DanceHallStudiosSchools => 7911,
            MerchantCategories::DatingEscortServices => 7273,
            MerchantCategories::DentistsOrthodontists => 8021,
            MerchantCategories::DepartmentStores => 5311,
            MerchantCategories::DetectiveAgencies => 7393,
            MerchantCategories::DirectMarketingCatalogMerchant => 5964,
            MerchantCategories::DirectMarketingCombinationCatalogAndRetailMerchant => 5965,
            MerchantCategories::DirectMarketingInboundTelemarketing => 5967,
            MerchantCategories::DirectMarketingInsuranceServices => 5960,
            MerchantCategories::DirectMarketingOther => 5969,
            MerchantCategories::DirectMarketingOutboundTelemarketing => 5966,
            MerchantCategories::DirectMarketingSubscription => 5968,
            MerchantCategories::DirectMarketingTravel => 5962,
            MerchantCategories::DiscountStores => 5310,
            MerchantCategories::Doctors => 8011,
            MerchantCategories::DoorToDoorSales => 5963,
            MerchantCategories::DraperyWindowCoveringAndUpholsteryStores => 5714,
            MerchantCategories::DrinkingPlaces => 5813,
            MerchantCategories::DrugStoresAndPharmacies => 5912,
            MerchantCategories::DrugsDrugProprietariesAndDruggistSundries => 5122,
            MerchantCategories::DryCleaners => 7216,
            MerchantCategories::DurableGoods => 5099,
            MerchantCategories::DutyFreeStores => 5309,
            MerchantCategories::EatingPlacesRestaurants => 5812,
            MerchantCategories::EducationalServices => 8299,
            MerchantCategories::ElectricRazorStores => 5997,
            MerchantCategories::ElectricalPartsAndEquipment => 5065,
            MerchantCategories::ElectricalServices => 1731,
            MerchantCategories::ElectronicsRepairShops => 7622,
            MerchantCategories::ElectronicsStores => 5732,
            MerchantCategories::ElementarySecondarySchools => 8211,
            MerchantCategories::EmploymentTempAgencies => 7361,
            MerchantCategories::EquipmentRental => 7394,
            MerchantCategories::ExterminatingServices => 7342,
            MerchantCategories::FamilyClothingStores => 5651,
            MerchantCategories::FastFoodRestaurants => 5814,
            MerchantCategories::FinancialInstitutions => 6012,
            MerchantCategories::FinesGovernmentAdministrativeEntities => 9222,
            MerchantCategories::FireplaceFireplaceScreensAndAccessoriesStores => 5718,
            MerchantCategories::FloorCoveringStores => 5713,
            MerchantCategories::Florists => 5992,
            MerchantCategories::FloristsSuppliesNurseryStockAndFlowers => 5193,
            MerchantCategories::FreezerAndLockerMeatProvisioners => 5422,
            MerchantCategories::FuelDealersNonAutomotive => 5983,
            MerchantCategories::FuneralServicesCrematories => 7261,
            MerchantCategories::FurnitureRepairRefinishing => 7641,
            MerchantCategories::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => 5712,
            MerchantCategories::FurriersAndFurShops => 5681,
            MerchantCategories::GeneralServices => 1520,
            MerchantCategories::GiftCardNoveltyAndSouvenirShops => 5947,
            MerchantCategories::GlassPaintAndWallpaperStores => 5231,
            MerchantCategories::GlasswareCrystalStores => 5950,
            MerchantCategories::GolfCoursesPublic => 7992,
            MerchantCategories::GovernmentServices => 9399,
            MerchantCategories::GroceryStoresSupermarkets => 5411,
            MerchantCategories::HardwareStores => 5251,
            MerchantCategories::HardwareEquipmentAndSupplies => 5072,
            MerchantCategories::HealthAndBeautySpas => 7298,
            MerchantCategories::HearingAidsSalesAndSupplies => 5975,
            MerchantCategories::HeatingPlumbingAC => 1711,
            MerchantCategories::HobbyToyAndGameShops => 5945,
            MerchantCategories::HomeSupplyWarehouseStores => 5200,
            MerchantCategories::Hospitals => 8062,
            MerchantCategories::HotelsMotelsAndResorts => 7011,
            MerchantCategories::HouseholdApplianceStores => 5722,
            MerchantCategories::IndustrialSupplies => 5085,
            MerchantCategories::InformationRetrievalServices => 7375,
            MerchantCategories::InsuranceDefault => 6399,
            MerchantCategories::InsuranceUnderwritingPremiums => 6300,
            MerchantCategories::IntraCompanyPurchases => 9950,
            MerchantCategories::JewelryStoresWatchesClocksAndSilverwareStores => 5944,
            MerchantCategories::LandscapingServices => 780,
            MerchantCategories::Laundries => 7211,
            MerchantCategories::LaundryCleaningServices => 7210,
            MerchantCategories::LegalServicesAttorneys => 8111,
            MerchantCategories::LuggageAndLeatherGoodsStores => 5948,
            MerchantCategories::LumberBuildingMaterialsStores => 5211,
            MerchantCategories::ManualCashDisburse => 6010,
            MerchantCategories::MarinasServiceAndSupplies => 4468,
            MerchantCategories::MasonryStoneworkAndPlaster => 1740,
            MerchantCategories::MassageParlors => 7297,
            MerchantCategories::MeansWomensClothingStores => 5691,
            MerchantCategories::MedicalServices => 8099,
            MerchantCategories::MedicalAndDentalLabs => 8071,
            MerchantCategories::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => 5047,
            MerchantCategories::MembershipOrganizations => 8699,
            MerchantCategories::MensAndBoysClothingAndAccessoriesStores => 5611,
            MerchantCategories::MetalServiceCenters => 5051,
            MerchantCategories::MiscellaneousApparelAndAccessoryShops => 5699,
            MerchantCategories::MiscellaneousAutoDealers => 5599,
            MerchantCategories::MiscellaneousBusinessServices => 7399,
            MerchantCategories::MiscellaneousFoodStores => 5499,
            MerchantCategories::MiscellaneousGeneralMerchandise => 5399,
            MerchantCategories::MiscellaneousGeneralServices => 7299,
            MerchantCategories::MiscellaneousHomeFurnishingSpecialtyStores => 5719,
            MerchantCategories::MiscellaneousPublishingAndPrinting => 2741,
            MerchantCategories::MiscellaneousRecreationServices => 7999,
            MerchantCategories::MiscellaneousRepairShops => 7699,
            MerchantCategories::MiscellaneousSpecialtyRetail => 5999,
            MerchantCategories::MobileHomeDealers => 5271,
            MerchantCategories::MotionPictureTheaters => 7832,
            MerchantCategories::MotorFreightCarriersAndTrucking => 4214,
            MerchantCategories::MotorHomesDealers => 5592,
            MerchantCategories::MotorVehicleSuppliesAndNewParts => 5013,
            MerchantCategories::MotorcycleShopsAndDealers => 5571,
            MerchantCategories::MotorcycleShopsDealers => 5561,
            MerchantCategories::MusicStoresMusicalInstrumentsPianosAndSheetMusic => 5733,
            MerchantCategories::NewsDealersAndNewsstands => 5994,
            MerchantCategories::NonFiMoneyOrders => 6051,
            MerchantCategories::NonFiStoredValueCardPurchaseLoad => 6540,
            MerchantCategories::NondurableGoods => 5199,
            MerchantCategories::NurseriesLawnAndGardenSupplyStores => 5261,
            MerchantCategories::NursingPersonalCare => 8050,
            MerchantCategories::OfficeAndCommercialFurniture => 5021,
            MerchantCategories::OpticiansEyeglasses => 8043,
            MerchantCategories::OptometristsOphthalmologist => 8042,
            MerchantCategories::OrthopedicGoodsProstheticDevices => 5976,
            MerchantCategories::Osteopaths => 8031,
            MerchantCategories::PackageStoresBeerWineAndLiquor => 5921,
            MerchantCategories::PaintsVarnishesAndSupplies => 5198,
            MerchantCategories::ParkingLotsGarages => 7523,
            MerchantCategories::PassengerRailways => 4112,
            MerchantCategories::PawnShops => 5933,
            MerchantCategories::PetShopsPetFoodAndSupplies => 5995,
            MerchantCategories::PetroleumAndPetroleumProducts => 5172,
            MerchantCategories::PhotoDeveloping => 7395,
            MerchantCategories::PhotographicStudios => 7221,
            MerchantCategories::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => 5044,
            MerchantCategories::PictureVideoProduction => 7829,
            MerchantCategories::PieceGoodsNotionsAndOtherDryGoods => 5131,
            MerchantCategories::PlumbingHeatingEquipmentAndSupplies => 5074,
            MerchantCategories::PoliticalOrganizations => 8651,
            MerchantCategories::PostalServicesGovernmentOnly => 9402,
            MerchantCategories::PreciousStonesAndMetalsWatchesAndJewelry => 5094,
            MerchantCategories::ProfessionalServices => 8999,
            MerchantCategories::PublicWarehousingAndStorage => 4225,
            MerchantCategories::QuickCopyReproAndBlueprint => 7338,
            MerchantCategories::Railroads => 4011,
            MerchantCategories::RealEstateAgentsAndManagersRentals => 6513,
            MerchantCategories::RecordStores => 5735,
            MerchantCategories::RecreationalVehicleRentals => 7519,
            MerchantCategories::ReligiousGoodsStores => 5973,
            MerchantCategories::ReligiousOrganizations => 8661,
            MerchantCategories::RoofingSidingSheetMetal => 1761,
            MerchantCategories::SecretarialSupportServices => 7339,
            MerchantCategories::SecurityBrokersDealers => 6211,
            MerchantCategories::ServiceStations => 5541,
            MerchantCategories::SewingNeedleworkFabricAndPieceGoodsStores => 5949,
            MerchantCategories::ShoeRepairHatCleaning => 7251,
            MerchantCategories::ShoeStores => 5661,
            MerchantCategories::SmallApplianceRepair => 7629,
            MerchantCategories::SnowmobileDealers => 5598,
            MerchantCategories::SpecialTradeServices => 1799,
            MerchantCategories::SpecialtyCleaning => 2842,
            MerchantCategories::SportingGoodsStores => 5941,
            MerchantCategories::SportingRecreationCamps => 7032,
            MerchantCategories::SportsClubsFields => 7941,
            MerchantCategories::SportsAndRidingApparelStores => 5655,
            MerchantCategories::StampAndCoinStores => 5972,
            MerchantCategories::StationaryOfficeSuppliesPrintingAndWritingPaper => 5111,
            MerchantCategories::StationeryStoresOfficeAndSchoolSupplyStores => 5943,
            MerchantCategories::SwimmingPoolsSales => 5996,
            MerchantCategories::TUiTravelGermany => 4723,
            MerchantCategories::TailorsAlterations => 5697,
            MerchantCategories::TaxPaymentsGovernmentAgencies => 9311,
            MerchantCategories::TaxPreparationServices => 7276,
            MerchantCategories::TaxicabsLimousines => 4121,
            MerchantCategories::TelecommunicationEquipmentAndTelephoneSales => 4812,
            MerchantCategories::TelecommunicationServices => 4814,
            MerchantCategories::TelegraphServices => 4821,
            MerchantCategories::TentAndAwningShops => 5998,
            MerchantCategories::TestingLaboratories => 8734,
            MerchantCategories::TheatricalTicketAgencies => 7922,
            MerchantCategories::Timeshares => 7012,
            MerchantCategories::TireRetreadingAndRepair => 7534,
            MerchantCategories::TollsBridgeFees => 4784,
            MerchantCategories::TouristAttractionsAndExhibits => 7991,
            MerchantCategories::TowingServices => 7549,
            MerchantCategories::TrailerParksCampgrounds => 7033,
            MerchantCategories::TransportationServices => 4789,
            MerchantCategories::TravelAgenciesTourOperators => 4722,
            MerchantCategories::TruckStopIteration => 7511,
            MerchantCategories::TruckUtilityTrailerRentals => 7513,
            MerchantCategories::TypesettingPlateMakingAndRelatedServices => 2791,
            MerchantCategories::TypewriterStores => 5978,
            MerchantCategories::USFederalGovernmentAgenciesOrDepartments => 9405,
            MerchantCategories::UniformsCommercialClothing => 5137,
            MerchantCategories::UsedMerchandiseAndSecondhandStores => 5931,
            MerchantCategories::Utilities => 4900,
            MerchantCategories::VarietyStores => 5331,
            MerchantCategories::VeterinaryServices => 742,
            MerchantCategories::VideoAmusementGameSupplies => 7993,
            MerchantCategories::VideoGameArcades => 7994,
            MerchantCategories::VideoTapeRentalStores => 7841,
            MerchantCategories::VocationalTradeSchools => 8249,
            MerchantCategories::WatchJewelryRepair => 7631,
            MerchantCategories::WeldingRepair => 7692,
            MerchantCategories::WholesaleClubs => 5300,
            MerchantCategories::WigAndToupeeStores => 5698,
            MerchantCategories::WiresMoneyOrders => 4829,
            MerchantCategories::WomensAccessoryAndSpecialtyShops => 5631,
            MerchantCategories::WomensReadyToWearStores => 5621,
            MerchantCategories::WreckingAndSalvageYards => 5935,
            MerchantCategories::Unknown => 0,
        }
    }

    pub fn from_mcc(mcc: i32) -> MerchantCategories {
        match mcc {
            7623 => MerchantCategories::AcRefrigerationRepair,
            8931 => MerchantCategories::AccountingBookkeepingServices,
            7311 => MerchantCategories::AdvertisingServices,
            763 => MerchantCategories::AgriculturalCooperative,
            4511 => MerchantCategories::AirlinesAirCarriers,
            4582 => MerchantCategories::AirportsFlyingFields,
            4119 => MerchantCategories::AmbulanceServices,
            7996 => MerchantCategories::AmusementParksCarnivals,
            5937 => MerchantCategories::AntiqueReproductions,
            5932 => MerchantCategories::AntiqueShops,
            7998 => MerchantCategories::Aquariums,
            8911 => MerchantCategories::ArchitecturalSurveyingServices,
            5971 => MerchantCategories::ArtDealersAndGalleries,
            5970 => MerchantCategories::ArtistsSupplyAndCraftShops,
            7531 => MerchantCategories::AutoBodyRepairShops,
            7535 => MerchantCategories::AutoPaintShops,
            7538 => MerchantCategories::AutoServiceShops,
            5531 => MerchantCategories::AutoAndHomeSupplyStores,
            6011 => MerchantCategories::AutomatedCashDisburse,
            5542 => MerchantCategories::AutomatedFuelDispensers,
            8675 => MerchantCategories::AutomobileAssociations,
            5533 => MerchantCategories::AutomotivePartsAndAccessoriesStores,
            5532 => MerchantCategories::AutomotiveTireStores,
            9223 => MerchantCategories::BailAndBondPayments,
            5462 => MerchantCategories::Bakeries,
            7929 => MerchantCategories::BandsOrchestras,
            7230 => MerchantCategories::BarberAndBeautyShops,
            7995 => MerchantCategories::BettingCasinoGambling,
            5940 => MerchantCategories::BicycleShops,
            7932 => MerchantCategories::BilliardPoolEstablishments,
            5551 => MerchantCategories::BoatDealers,
            4457 => MerchantCategories::BoatRentalsAndLeases,
            5942 => MerchantCategories::BookStores,
            5192 => MerchantCategories::BooksPeriodicalsAndNewspapers,
            7933 => MerchantCategories::BowlingAlleys,
            4131 => MerchantCategories::BusLines,
            8244 => MerchantCategories::BusinessSecretarialSchools,
            7278 => MerchantCategories::BuyingShoppingServices,
            4899 => MerchantCategories::CableSatelliteAndOtherPayTelevisionAndRadio,
            5946 => MerchantCategories::CameraAndPhotographicSupplyStores,
            5441 => MerchantCategories::CandyNutAndConfectioneryStores,
            7512 => MerchantCategories::CarRentalAgencies,
            7542 => MerchantCategories::CarWashes,
            5511 => MerchantCategories::CarAndTruckDealersNewUsed,
            5521 => MerchantCategories::CarAndTruckDealersUsedOnly,
            1750 => MerchantCategories::CarpentryServices,
            7217 => MerchantCategories::CarpetUpholsteryCleaning,
            5811 => MerchantCategories::Caterers,
            8398 => MerchantCategories::CharitableAndSocialServiceOrganizationsFundraising,
            5169 => MerchantCategories::ChemicalsAndAlliedProducts,
            5641 => MerchantCategories::ChidrensAndInfantsWearStores,
            8351 => MerchantCategories::ChildCareServices,
            8049 => MerchantCategories::ChiropodistsPodiatrists,
            8041 => MerchantCategories::Chiropractors,
            5993 => MerchantCategories::CigarStoresAndStands,
            8641 => MerchantCategories::CivicSocialFraternalAssociations,
            7349 => MerchantCategories::CleaningAndMaintenance,
            7296 => MerchantCategories::ClothingRental,
            8220 => MerchantCategories::CollegesUniversities,
            5046 => MerchantCategories::CommercialEquipment,
            5139 => MerchantCategories::CommercialFootwear,
            7333 => MerchantCategories::CommercialPhotographyArtAndGraphics,
            4111 => MerchantCategories::CommuterTransportAndFerries,
            4816 => MerchantCategories::ComputerNetworkServices,
            7372 => MerchantCategories::ComputerProgramming,
            7379 => MerchantCategories::ComputerRepair,
            5734 => MerchantCategories::ComputerSoftwareStores,
            5045 => MerchantCategories::ComputersPeripheralsAndSoftware,
            1771 => MerchantCategories::ConcreteWorkServices,
            5039 => MerchantCategories::ConstructionMaterials,
            7392 => MerchantCategories::ConsultingPublicRelations,
            8241 => MerchantCategories::CorrespondenceSchools,
            5977 => MerchantCategories::CosmeticStores,
            7277 => MerchantCategories::CounselingServices,
            7997 => MerchantCategories::CountryClubs,
            4215 => MerchantCategories::CourierServices,
            9211 => MerchantCategories::CourtCosts,
            7321 => MerchantCategories::CreditReportingAgencies,
            4411 => MerchantCategories::CruiseLines,
            5451 => MerchantCategories::DairyProductsStores,
            7911 => MerchantCategories::DanceHallStudiosSchools,
            7273 => MerchantCategories::DatingEscortServices,
            8021 => MerchantCategories::DentistsOrthodontists,
            5311 => MerchantCategories::DepartmentStores,
            7393 => MerchantCategories::DetectiveAgencies,
            5964 => MerchantCategories::DirectMarketingCatalogMerchant,
            5965 => MerchantCategories::DirectMarketingCombinationCatalogAndRetailMerchant,
            5967 => MerchantCategories::DirectMarketingInboundTelemarketing,
            5960 => MerchantCategories::DirectMarketingInsuranceServices,
            5969 => MerchantCategories::DirectMarketingOther,
            5966 => MerchantCategories::DirectMarketingOutboundTelemarketing,
            5968 => MerchantCategories::DirectMarketingSubscription,
            5962 => MerchantCategories::DirectMarketingTravel,
            5310 => MerchantCategories::DiscountStores,
            8011 => MerchantCategories::Doctors,
            5963 => MerchantCategories::DoorToDoorSales,
            5714 => MerchantCategories::DraperyWindowCoveringAndUpholsteryStores,
            5813 => MerchantCategories::DrinkingPlaces,
            5912 => MerchantCategories::DrugStoresAndPharmacies,
            5122 => MerchantCategories::DrugsDrugProprietariesAndDruggistSundries,
            7216 => MerchantCategories::DryCleaners,
            5099 => MerchantCategories::DurableGoods,
            5309 => MerchantCategories::DutyFreeStores,
            5812 => MerchantCategories::EatingPlacesRestaurants,
            8299 => MerchantCategories::EducationalServices,
            5997 => MerchantCategories::ElectricRazorStores,
            5065 => MerchantCategories::ElectricalPartsAndEquipment,
            1731 => MerchantCategories::ElectricalServices,
            7622 => MerchantCategories::ElectronicsRepairShops,
            5732 => MerchantCategories::ElectronicsStores,
            8211 => MerchantCategories::ElementarySecondarySchools,
            7361 => MerchantCategories::EmploymentTempAgencies,
            7394 => MerchantCategories::EquipmentRental,
            7342 => MerchantCategories::ExterminatingServices,
            5651 => MerchantCategories::FamilyClothingStores,
            5814 => MerchantCategories::FastFoodRestaurants,
            6012 => MerchantCategories::FinancialInstitutions,
            9222 => MerchantCategories::FinesGovernmentAdministrativeEntities,
            5718 => MerchantCategories::FireplaceFireplaceScreensAndAccessoriesStores,
            5713 => MerchantCategories::FloorCoveringStores,
            5992 => MerchantCategories::Florists,
            5193 => MerchantCategories::FloristsSuppliesNurseryStockAndFlowers,
            5422 => MerchantCategories::FreezerAndLockerMeatProvisioners,
            5983 => MerchantCategories::FuelDealersNonAutomotive,
            7261 => MerchantCategories::FuneralServicesCrematories,
            7641 => MerchantCategories::FurnitureRepairRefinishing,
            5712 => MerchantCategories::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
            5681 => MerchantCategories::FurriersAndFurShops,
            1520 => MerchantCategories::GeneralServices,
            5947 => MerchantCategories::GiftCardNoveltyAndSouvenirShops,
            5231 => MerchantCategories::GlassPaintAndWallpaperStores,
            5950 => MerchantCategories::GlasswareCrystalStores,
            7992 => MerchantCategories::GolfCoursesPublic,
            9399 => MerchantCategories::GovernmentServices,
            5411 => MerchantCategories::GroceryStoresSupermarkets,
            5251 => MerchantCategories::HardwareStores,
            5072 => MerchantCategories::HardwareEquipmentAndSupplies,
            7298 => MerchantCategories::HealthAndBeautySpas,
            5975 => MerchantCategories::HearingAidsSalesAndSupplies,
            1711 => MerchantCategories::HeatingPlumbingAC,
            5945 => MerchantCategories::HobbyToyAndGameShops,
            5200 => MerchantCategories::HomeSupplyWarehouseStores,
            8062 => MerchantCategories::Hospitals,
            7011 => MerchantCategories::HotelsMotelsAndResorts,
            5722 => MerchantCategories::HouseholdApplianceStores,
            5085 => MerchantCategories::IndustrialSupplies,
            7375 => MerchantCategories::InformationRetrievalServices,
            6399 => MerchantCategories::InsuranceDefault,
            6300 => MerchantCategories::InsuranceUnderwritingPremiums,
            9950 => MerchantCategories::IntraCompanyPurchases,
            5944 => MerchantCategories::JewelryStoresWatchesClocksAndSilverwareStores,
            780 => MerchantCategories::LandscapingServices,
            7211 => MerchantCategories::Laundries,
            7210 => MerchantCategories::LaundryCleaningServices,
            8111 => MerchantCategories::LegalServicesAttorneys,
            5948 => MerchantCategories::LuggageAndLeatherGoodsStores,
            5211 => MerchantCategories::LumberBuildingMaterialsStores,
            6010 => MerchantCategories::ManualCashDisburse,
            4468 => MerchantCategories::MarinasServiceAndSupplies,
            1740 => MerchantCategories::MasonryStoneworkAndPlaster,
            7297 => MerchantCategories::MassageParlors,
            5691 => MerchantCategories::MeansWomensClothingStores,
            8099 => MerchantCategories::MedicalServices,
            8071 => MerchantCategories::MedicalAndDentalLabs,
            5047 => MerchantCategories::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
            8699 => MerchantCategories::MembershipOrganizations,
            5611 => MerchantCategories::MensAndBoysClothingAndAccessoriesStores,
            5051 => MerchantCategories::MetalServiceCenters,
            5699 => MerchantCategories::MiscellaneousApparelAndAccessoryShops,
            5599 => MerchantCategories::MiscellaneousAutoDealers,
            7399 => MerchantCategories::MiscellaneousBusinessServices,
            5499 => MerchantCategories::MiscellaneousFoodStores,
            5399 => MerchantCategories::MiscellaneousGeneralMerchandise,
            7299 => MerchantCategories::MiscellaneousGeneralServices,
            5719 => MerchantCategories::MiscellaneousHomeFurnishingSpecialtyStores,
            2741 => MerchantCategories::MiscellaneousPublishingAndPrinting,
            7999 => MerchantCategories::MiscellaneousRecreationServices,
            7699 => MerchantCategories::MiscellaneousRepairShops,
            5999 => MerchantCategories::MiscellaneousSpecialtyRetail,
            5271 => MerchantCategories::MobileHomeDealers,
            7832 => MerchantCategories::MotionPictureTheaters,
            4214 => MerchantCategories::MotorFreightCarriersAndTrucking,
            5592 => MerchantCategories::MotorHomesDealers,
            5013 => MerchantCategories::MotorVehicleSuppliesAndNewParts,
            5571 => MerchantCategories::MotorcycleShopsAndDealers,
            5561 => MerchantCategories::MotorcycleShopsDealers,
            5733 => MerchantCategories::MusicStoresMusicalInstrumentsPianosAndSheetMusic,
            5994 => MerchantCategories::NewsDealersAndNewsstands,
            6051 => MerchantCategories::NonFiMoneyOrders,
            6540 => MerchantCategories::NonFiStoredValueCardPurchaseLoad,
            5199 => MerchantCategories::NondurableGoods,
            5261 => MerchantCategories::NurseriesLawnAndGardenSupplyStores,
            8050 => MerchantCategories::NursingPersonalCare,
            5021 => MerchantCategories::OfficeAndCommercialFurniture,
            8043 => MerchantCategories::OpticiansEyeglasses,
            8042 => MerchantCategories::OptometristsOphthalmologist,
            5976 => MerchantCategories::OrthopedicGoodsProstheticDevices,
            8031 => MerchantCategories::Osteopaths,
            5921 => MerchantCategories::PackageStoresBeerWineAndLiquor,
            5198 => MerchantCategories::PaintsVarnishesAndSupplies,
            7523 => MerchantCategories::ParkingLotsGarages,
            4112 => MerchantCategories::PassengerRailways,
            5933 => MerchantCategories::PawnShops,
            5995 => MerchantCategories::PetShopsPetFoodAndSupplies,
            5172 => MerchantCategories::PetroleumAndPetroleumProducts,
            7395 => MerchantCategories::PhotoDeveloping,
            7221 => MerchantCategories::PhotographicStudios,
            5044 => MerchantCategories::PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
            7829 => MerchantCategories::PictureVideoProduction,
            5131 => MerchantCategories::PieceGoodsNotionsAndOtherDryGoods,
            5074 => MerchantCategories::PlumbingHeatingEquipmentAndSupplies,
            8651 => MerchantCategories::PoliticalOrganizations,
            9402 => MerchantCategories::PostalServicesGovernmentOnly,
            5094 => MerchantCategories::PreciousStonesAndMetalsWatchesAndJewelry,
            8999 => MerchantCategories::ProfessionalServices,
            4225 => MerchantCategories::PublicWarehousingAndStorage,
            7338 => MerchantCategories::QuickCopyReproAndBlueprint,
            4011 => MerchantCategories::Railroads,
            6513 => MerchantCategories::RealEstateAgentsAndManagersRentals,
            5735 => MerchantCategories::RecordStores,
            7519 => MerchantCategories::RecreationalVehicleRentals,
            5973 => MerchantCategories::ReligiousGoodsStores,
            8661 => MerchantCategories::ReligiousOrganizations,
            1761 => MerchantCategories::RoofingSidingSheetMetal,
            7339 => MerchantCategories::SecretarialSupportServices,
            6211 => MerchantCategories::SecurityBrokersDealers,
            5541 => MerchantCategories::ServiceStations,
            5949 => MerchantCategories::SewingNeedleworkFabricAndPieceGoodsStores,
            7251 => MerchantCategories::ShoeRepairHatCleaning,
            5661 => MerchantCategories::ShoeStores,
            7629 => MerchantCategories::SmallApplianceRepair,
            5598 => MerchantCategories::SnowmobileDealers,
            1799 => MerchantCategories::SpecialTradeServices,
            2842 => MerchantCategories::SpecialtyCleaning,
            5941 => MerchantCategories::SportingGoodsStores,
            7032 => MerchantCategories::SportingRecreationCamps,
            7941 => MerchantCategories::SportsClubsFields,
            5655 => MerchantCategories::SportsAndRidingApparelStores,
            5972 => MerchantCategories::StampAndCoinStores,
            5111 => MerchantCategories::StationaryOfficeSuppliesPrintingAndWritingPaper,
            5943 => MerchantCategories::StationeryStoresOfficeAndSchoolSupplyStores,
            5996 => MerchantCategories::SwimmingPoolsSales,
            4723 => MerchantCategories::TUiTravelGermany,
            5697 => MerchantCategories::TailorsAlterations,
            9311 => MerchantCategories::TaxPaymentsGovernmentAgencies,
            7276 => MerchantCategories::TaxPreparationServices,
            4121 => MerchantCategories::TaxicabsLimousines,
            4812 => MerchantCategories::TelecommunicationEquipmentAndTelephoneSales,
            4814 => MerchantCategories::TelecommunicationServices,
            4821 => MerchantCategories::TelegraphServices,
            5998 => MerchantCategories::TentAndAwningShops,
            8734 => MerchantCategories::TestingLaboratories,
            7922 => MerchantCategories::TheatricalTicketAgencies,
            7012 => MerchantCategories::Timeshares,
            7534 => MerchantCategories::TireRetreadingAndRepair,
            4784 => MerchantCategories::TollsBridgeFees,
            7991 => MerchantCategories::TouristAttractionsAndExhibits,
            7549 => MerchantCategories::TowingServices,
            7033 => MerchantCategories::TrailerParksCampgrounds,
            4789 => MerchantCategories::TransportationServices,
            4722 => MerchantCategories::TravelAgenciesTourOperators,
            7511 => MerchantCategories::TruckStopIteration,
            7513 => MerchantCategories::TruckUtilityTrailerRentals,
            2791 => MerchantCategories::TypesettingPlateMakingAndRelatedServices,
            5978 => MerchantCategories::TypewriterStores,
            9405 => MerchantCategories::USFederalGovernmentAgenciesOrDepartments,
            5137 => MerchantCategories::UniformsCommercialClothing,
            5931 => MerchantCategories::UsedMerchandiseAndSecondhandStores,
            4900 => MerchantCategories::Utilities,
            5331 => MerchantCategories::VarietyStores,
            742 => MerchantCategories::VeterinaryServices,
            7993 => MerchantCategories::VideoAmusementGameSupplies,
            7994 => MerchantCategories::VideoGameArcades,
            7841 => MerchantCategories::VideoTapeRentalStores,
            8249 => MerchantCategories::VocationalTradeSchools,
            7631 => MerchantCategories::WatchJewelryRepair,
            7692 => MerchantCategories::WeldingRepair,
            5300 => MerchantCategories::WholesaleClubs,
            5698 => MerchantCategories::WigAndToupeeStores,
            4829 => MerchantCategories::WiresMoneyOrders,
            5631 => MerchantCategories::WomensAccessoryAndSpecialtyShops,
            5621 => MerchantCategories::WomensReadyToWearStores,
            5935 => MerchantCategories::WreckingAndSalvageYards,
            _ => MerchantCategories::Unknown,
        }
    }
}
