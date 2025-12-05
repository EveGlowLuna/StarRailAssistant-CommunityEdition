// English language pack
export default {
  // Navigation menu
  nav: {
    home: 'Home',
    tasks: 'Tasks',
    extensions: 'Extensions',
    console: 'Console',
    announcement: 'Announcement',
    settings: 'Settings'
  },

  // Home page
  home: {
    title: 'Home',
    coreStatus: 'Core Status',
    notRunning: 'Not Running',
    running: 'Core Running',
    taskRunning: 'Task Running',
    error: 'Core Error',
    startTask: 'Start Task',
    stopTask: 'Stop Task',
    restartCore: 'Restart Core',
    startCore: 'Start Core',
    configList: 'Config List',
    selectAll: 'Select All',
    deselectAll: 'Deselect All',
    invertSelection: 'Invert Selection',
    deleteSelected: 'Delete Selected',
    newConfig: 'New Config',
    refresh: 'Refresh',
    configName: 'Config Name',
    create: 'Create',
    cancel: 'Cancel',
    configManagement: 'Config Management',
    quickAccess: 'Quick Access',
    versionUpdate: 'Version Update',
    versionUpdateDesc: 'Check latest version',
    announcement: 'Announcement',
    announcementDesc: 'View announcements',
    tutorial: 'Tutorial',
    tutorialDesc: 'View tutorials',
    notifications: {
      cannotDeleteDefault: 'Default config cannot be deleted.',
      taskStopped: 'Task stopped',
      createShortcut: 'Create desktop shortcut for StarRailAssistant?',
      yes: 'Yes',
      no: 'No',
      noRemind: 'Don\'t remind again',
      shortcutCreated: 'Desktop shortcut created successfully',
      shortcutFailed: 'Failed to create desktop shortcut',
      stopTaskFailed: 'Failed to stop task',
      selectConfigFirst: 'Please select a config first',
      taskStarted: 'Task started: {names}',
      executeTaskFailed: 'Task execution failed',
      taskCompleted: 'Task completed',
      coreRestarted: 'Core restarted successfully',
      coreStarted: 'Core started successfully',
      coreRestartFailed: 'Failed to restart core, please check if SRA-cli exists or troubleshoot manually'
    }
  },

  // Tasks page
  tasks: {
    title: 'Tasks',
    selectConfig: 'Select Config',
    saveConfig: 'Save Config',
    newConfig: 'New Config',
    saveAndExecute: 'Save & Execute',
    configName: 'Config Name',
    configNamePlaceholder: 'Enter config name',
    cancel: 'Cancel',
    confirm: 'Confirm',
    
    // Start Game
    startGame: {
      title: 'Start Game',
      gamePath: 'Game Path',
      selectPath: 'Select Path',
      channel: 'Channel',
      officialServer: 'Official Server',
      bilibiliServer: 'Bilibili Server',
      autoLogin: 'Auto Login',
      autoLoginHint: 'By checking Auto Login, you acknowledge that you have read and agree to the User Agreement and Privacy Policy',
      alwaysLogin: 'Always Login',
      username: 'Username',
      usernamePlaceholder: 'Enter username',
      password: 'Password',
      passwordPlaceholder: 'Enter password'
    },

    // Trailblaze Power
    trailblazePower: {
      title: 'Trailblaze Power',
      taskList: 'Task List',
      addTask: 'Add Task',
      selectLevel: 'Select Level',
      continuousBattle: 'Continuous Battle',
      runTimes: 'Run Times',
      add: 'Add',
      times: 'Times',
      useSupport: 'Use Support',
      useSkill: 'Use Skill',
      replenishPower: 'Replenish Trailblaze Power',
      replenishTimes: 'Replenish Times',
      replenishMethod: 'Replenish Method',
      reservedPower: 'Reserved Trailblaze Power',
      fuel: 'Fuel',
      stellarJade: 'Stellar Jade',
      changeLineup: 'Change Lineup',
      lineupCheck: 'Lineup Check',
      reLineup: 'Re-lineup',
      useSkillEntry: 'Use Skill Entry',
      selectInstance: '---Select Instance---',
      
      // Level categories
      categories: {
        calyxGolden: 'Calyx (Golden)',
        calyxCrimson: 'Calyx (Crimson)',
        stagnantShadow: 'Stagnant Shadow',
        cavern: 'Cavern of Corrosion',
        echoOfWar: 'Echo of War',
        ornament: 'Ornament Extraction'
      }
    },

    // Trailblaze Power Level Names
    trailblazePowerLevels: {
      // Ornament Extraction
      ornament: {
        moonlitBlood: 'Moonlit Blood (Fairy/Seaside)',
        unceasingStrife: 'Unceasing Strife (Bone Collector/Giant Tree)',
        famishedWorker: 'Famished Worker (Lusaka/Banana Paradise)',
        eternalComedy: 'Eternal Comedy (Dulan/Calamity Fire)',
        toSweetDreams: 'To Sweet Dreams (Tsigonia/Izumo)',
        pouringBlades: 'Pouring Blades (Gram/Penacony)',
        fruitOfEvil: 'Fruit of Evil (Starry/Keel)',
        permafrost: 'Permafrost (Belobog/Salsotto)',
        gentleWords: 'Gentle Words (Fleet/Difference Engine)',
        smeltedHeart: 'Smelted Heart (Talia/Vonwacq)',
        untoppledWalls: 'Untoppled Walls (Space Station/Xianzhou)'
      },
      // Calyx (Golden)
      calyxGolden: {
        budOfMemoriesAmphoreus: 'Bud of Memories (Amphoreus)',
        budOfAetherAmphoreus: 'Bud of Aether (Amphoreus)',
        budOfTreasuresAmphoreus: 'Bud of Treasures (Amphoreus)',
        budOfMemoriesPenacony: 'Bud of Memories (Penacony)',
        budOfAetherPenacony: 'Bud of Aether (Penacony)',
        budOfTreasuresPenacony: 'Bud of Treasures (Penacony)',
        budOfMemoriesLuofu: 'Bud of Memories (The Xianzhou Luofu)',
        budOfAetherLuofu: 'Bud of Aether (The Xianzhou Luofu)',
        budOfTreasuresLuofu: 'Bud of Treasures (The Xianzhou Luofu)',
        budOfMemoriesJarilo: 'Bud of Memories (Jarilo-VI)',
        budOfAetherJarilo: 'Bud of Aether (Jarilo-VI)',
        budOfTreasuresJarilo: 'Bud of Treasures (Jarilo-VI)'
      },
      // Calyx (Crimson)
      calyxCrimson: {
        moonRageFang: 'Moon Rage Fang (Destruction)',
        worldbreakerBlade: 'Worldbreaker Blade (Destruction)',
        divineAmber: 'Divine Amber (Preservation)',
        safeguardOfAmber: 'Safeguard of Amber (Preservation)',
        countertemporalShot: 'Countertemporal Shot (The Hunt)',
        arrowOfTheStarchaser: 'Arrow of the Starchaser (The Hunt)',
        myriadFruit: 'Myriad Fruit (Abundance)',
        flowerOfEternity: 'Flower of Eternity (Abundance)',
        exquisiteColoredDraft: 'Exquisite Colored Draft (Erudition)',
        keyOfWisdom: 'Key of Wisdom (Erudition)',
        heavenlyMelody: 'Heavenly Melody (Harmony)',
        stellarisSymphony: 'Stellaris Symphony (Harmony)',
        heavenIncinerator: 'Heaven Incinerator (Nihility)',
        obsidianOfObsession: 'Obsidian of Obsession (Nihility)',
        flowerOfAlaya: 'Flower of Alaya (Remembrance)'
      },
      // Stagnant Shadow
      stagnantShadow: {
        invasiveClot: 'Invasive Clot (Physical)',
        ipcWorkPermit: 'IPC Work Permit (Physical)',
        netherworldToken: 'Netherworld Token (Physical)',
        brokenTeethOfIronWolf: 'Broken Teeth of Iron Wolf (Physical)',
        radiantProminence: 'Radiant Prominence (Fire)',
        ragingHeart: 'Raging Heart (Fire)',
        searingSteelBlade: 'Searing Steel Blade (Fire)',
        endothermChitin: 'Endotherm Chitin (Fire)',
        seaSirensTornFin: 'Sea Siren\'s Torn Fin (Ice)',
        dreamFridge: 'Dream Fridge (Ice)',
        gelidChitin: 'Gelid Chitin (Ice)',
        hornOfSnow: 'Horn of Snow (Ice)',
        nailOfTheBeastCoffin: 'Nail of the Beast Coffin (Lightning)',
        shapeShiffersLightningStaff: 'Shape Shiffer\'s Lightning Staff (Lightning)',
        lightningCrownOfThePastShadow: 'Lightning Crown of the Past Shadow (Lightning)',
        charredBudOfTwilight: 'Charred Bud of Twilight (Wind)',
        aGlassOfTheBesottedEra: 'A Glass of the Besotted Era (Wind)',
        ascendantDebirs: 'Ascendant Debirs (Wind)',
        stormEye: 'Storm Eye (Wind)',
        darkveilMoonlight: 'Darkveil Moonlight (Quantum)',
        dreamFlamer: 'Dream Flamer (Quantum)',
        nailOfTheApe: 'Nail of the Ape (Quantum)',
        voidCastIron: 'Void Cast Iron (Quantum)',
        harbingerOfStrife: 'Harbinger of Strife (Imaginary)',
        chordalMirage: 'Chordal Mirage (Imaginary)',
        suppressingEdict: 'Suppressing Edict (Imaginary)',
        goldenCrownOfThePastShadow: 'Golden Crown of the Past Shadow (Imaginary)'
      },
      // Cavern of Corrosion
      cavern: {
        pathOfHiddenSalvation: 'Path of Hidden Salvation (Savior/Hermit)',
        pathOfThundersurge: 'Path of Thundersurge (Valkyrie/Captain)',
        pathOfAria: 'Path of Aria (Hero/Poet)',
        pathOfUncertainty: 'Path of Uncertainty (Priest/Scholar)',
        pathOfCavalier: 'Path of Cavalier (Iron Cavalry/Brave)',
        pathOfDreamdive: 'Path of Dreamdive (Dead Water/Clockmaker)',
        pathOfDarkness: 'Path of Darkness (Duke/Prisoner)',
        pathOfElixirSeekers: 'Path of Elixir Seekers (Planter/Messenger)',
        pathOfConflagration: 'Path of Conflagration (Fire Pioneer/Wastelander)',
        pathOfHolyHymn: 'Path of Holy Hymn (Holy Knight/Band)',
        pathOfProvidence: 'Path of Providence (Iron Guard/Quantum)',
        pathOfDrifting: 'Path of Drifting (Passerby/Musketeer)',
        pathOfJabbingPunch: 'Path of Jabbing Punch (Champion/Thief)',
        pathOfGelidWind: 'Path of Gelid Wind (Ice/Wind)'
      },
      // Echo of War
      echoOfWar: {
        glanceOfTwilight: 'Glance of Twilight',
        innerBeastsBattlefield: 'Inner Beast\'s Battlefield',
        salutationsOfAshenDreams: 'Salutations of Ashen Dreams',
        boreholePlanetsPastNightmares: 'Borehole Planet\'s Past Nightmares',
        divineSeed: 'Divine Seed',
        endOfTheEternalFreeze: 'End of the Eternal Freeze',
        destructionsBeginning: 'Destruction\'s Beginning',
        rustedCryptOfTheIronCarcass: 'Rusted Crypt of the Iron Carcass'
      }
    },

    // Receive Rewards
    receiveReward: {
      title: 'Receive Rewards',
      guide: 'Interastral Peace Guide',
      assignments: 'Assignments',
      mailbox: 'Mailbox',
      dailyTraining: 'Daily Training',
      namelessHonor: 'Nameless Honor',
      giftOfOdyssey: 'Gift of Odyssey',
      redemptionCode: 'Redemption Code',
      redeemCodes: 'Redemption Codes',
      redeemCodesPlaceholder: 'One code per line'
    },

    // Cosmic Strife
    cosmicStrife: {
      title: 'Cosmic Strife',
      enable: 'Enable',
      mode: 'Mode',
      clearLevel: 'Clear Level',
      divergentUniverse: 'Divergent Universe',
      currencyWars: 'Currency Wars',
      policy: 'Policy',
      proteanHero: 'Divergent Universe - Protean Hero',
      singleLayer: 'Single-layer brushing',
      standardGambit: 'Standard Gambit',
      overclockGambit: 'Overclock Gambit',
      runTimes: 'Run Times',
      trailblazer: 'Trailblazer',
      trailblazerPlaceholder: 'Enter Trailblazer name',
      zeroSumGame: 'Zero-Sum Game',
      hintText: 'Please complete manually once to clear all tutorial prompts'
    },

    // After Mission
    afterMission: {
      title: 'After Mission',
      exitGame: 'Exit Game',
      logout: 'Logout',
      sleep: 'Sleep',
      shutdown: 'Shutdown',
      exitApp: 'Exit App',
      none: 'No Action'
    },

    notifications: {
      configSaved: 'Config saved successfully',
      configSaveFailed: 'Failed to save config',
      taskStarted: 'Task started: {name}',
      executeFailed: 'Execution failed',
      configCreated: 'Config created successfully',
      configCreateFailed: 'Failed to create config',
      selectLevel: 'Please select a level'
    },

    // Task list related
    taskList: {
      singleTime: 'Single',
      execution: 'Execution',
      moveUp: 'Move Up',
      moveDown: 'Move Down',
      deleteTask: 'Delete Task'
    },

    // Task type names
    taskTypes: {
      ornament: 'Ornament Extraction',
      calyxGolden: 'Calyx (Golden)',
      calyxCrimson: 'Calyx (Crimson)',
      stagnantShadow: 'Stagnant Shadow',
      cavern: 'Cavern of Corrosion',
      echoOfWar: 'Echo of War'
    }
  },

  // Extensions page
  extensions: {
    title: 'Extensions',
    autoPlot: {
      title: 'Auto Plot',
      enable: 'Enable',
      enableDesc: 'Enable auto plot feature',
      skipPlot: 'Skip Plot',
      skipPlotDesc: 'Automatically skip plot content'
    },
    notifications: {
      autoPlotEnabled: 'Auto plot enabled',
      autoPlotDisabled: 'Auto plot disabled',
      skipPlotEnabled: 'Skip plot enabled',
      skipPlotDisabled: 'Skip plot disabled',
      toggleFailed: 'Toggle failed'
    }
  },

  // Console
  console: {
    title: 'Console',
    filterLevel: 'Filter Level',
    filterSource: 'Filter Source',
    clearConsole: 'Clear Console',
    exportLogs: 'Export Logs',
    inputPlaceholder: 'Enter command...',
    send: 'Send',
    sources: {
      frontend: 'Frontend',
      backend: 'Backend',
      process: 'Process'
    },
    notifications: {
      noLogsToExport: 'No logs to export',
      exportSuccess: 'Logs exported successfully',
      exportFailed: 'Failed to export logs',
      taskStarted: 'Custom task started'
    },
    sendFailed: 'Send failed',
    sendMessageFailed: 'Failed to send message',
    logs: {
      wallpaperDataLength: 'Wallpaper data length: {length}',
      startGetWallpaper: 'Starting get_wallpaper_base64 call',
      getWallpaperResult: 'get_wallpaper_base64 returned: {result}',
      hasData: 'has data',
      noData: 'no data',
      loadWallpaperSuccess: 'Wallpaper loaded successfully on app startup',
      noCustomWallpaper: 'No custom wallpaper, using default background',
      loadWallpaperFailed: 'Failed to load wallpaper: {error}',
      loadSettingsFailed: 'Failed to load settings: {error}',
      openWallpaperDialog: 'Opening wallpaper selection dialog',
      userCancelledWallpaper: 'User cancelled wallpaper selection',
      userSelectedWallpaper: 'User selected wallpaper: {path}',
      callSetWallpaper: 'Calling set_wallpaper command',
      wallpaperCached: 'Wallpaper cached to: {path}',
      callGetWallpaperBase64: 'Calling get_wallpaper_base64 command',
      getWallpaperBase64Result: 'get_wallpaper_base64 returned: {result}',
      hasDataWithSize: 'has data ({size} bytes)',
      startApplyWallpaper: 'Starting to apply wallpaper to background',
      wallpaperApplied: 'Wallpaper applied to background',
      getWallpaperBase64Empty: 'get_wallpaper_base64 returned empty data',
      startResetWallpaper: 'Starting to reset wallpaper',
      wallpaperStyleApplied: 'Wallpaper style applied',
      appContainerNotFound: 'Cannot find .app-container element',
      wallpaperStyleRemoved: 'Wallpaper style removed'
    }
  },

  // Announcement
  announcement: {
    title: 'Announcements',
    refresh: 'Refresh',
    mainAnnouncement: 'Main Announcement',
    releaseAnnouncement: 'Latest Release',
    otherAnnouncements: 'Other Announcements',
    loading: 'Loading...',
    error: 'Failed to load announcements: If you are not using a VPN, this is not your fault. The server may be down or under attack.',
    retry: 'Retry',
    noAnnouncements: 'No announcements available'
  },

  // Settings
  settings: {
    title: 'Settings',
    saveSettings: 'Save Settings',
    resetSettings: 'Reset Settings',
    
    theme: {
      title: 'Theme Settings',
      customWallpaper: 'Custom Wallpaper',
      selectWallpaper: 'Select Wallpaper',
      resetWallpaper: 'Reset Wallpaper'
    },
    
    interface: {
      title: 'Interface Settings',
      language: 'Interface Language',
      languageNote: 'To use automation features, your game interface must be set to Simplified Chinese',
      zoom: 'Screen Zoom',
      confidence: 'Image Recognition Confidence Threshold'
    },
    
    hotkeys: {
      title: 'In-Game Hotkeys',
      clickToSet: 'Click input box to set hotkey',
      pressKey: 'Press a key to set hotkey',
      activity: 'Activity',
      chronicle: 'Chronicle',
      warp: 'Warp',
      guide: 'Guide',
      map: 'Map',
      technique: 'Technique'
    },
    
    notifications: {
      title: 'Notifications',
      allowNotifications: 'Allow Notifications',
      allowSystemNotifications: 'System Notifications',
      allowEmailNotifications: 'Email Notifications',
      smtpSettings: 'SMTP Settings',
      smtpServer: 'SMTP Server',
      smtpPort: 'SMTP Port',
      emailSender: 'Sender Email',
      emailSenderPlaceholder: 'e.g.: 123456@qq.com',
      emailAuthCode: 'Email Auth Code',
      emailAuthCodePlaceholder: 'e.g.: 123456',
      emailReceiver: 'Receiver Email',
      emailReceiverPlaceholder: 'e.g.: 123456@qq.com',
      settingsSaved: 'Settings saved successfully',
      saveSettingsFailed: 'Failed to save settings',
      wallpaperSet: 'Wallpaper set successfully',
      selectWallpaperFailed: 'Failed to select wallpaper',
      wallpaperReset: 'Wallpaper reset to default',
      resetWallpaperFailed: 'Failed to reset wallpaper',
      unknownError: 'Unknown error'
    },
    
    app: {
      title: 'App Settings',
      enableStartupLaunch: 'Launch on Startup',
      enableMinimizeToTray: 'Minimize to Tray'
    },
    
    resetConfirm: 'Are you sure you want to reset all settings to default values?'
  },

  // Common
  common: {
    confirm: 'Confirm',
    cancel: 'Cancel',
    save: 'Save',
    delete: 'Delete',
    edit: 'Edit',
    close: 'Close',
    loading: 'Loading...',
    success: 'Success',
    error: 'Error',
    warning: 'Warning'
  }
}
