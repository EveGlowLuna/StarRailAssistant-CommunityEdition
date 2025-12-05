// 简体中文语言包
export default {
  // 导航菜单
  nav: {
    home: '主页',
    tasks: '任务',
    extensions: '扩展',
    console: '控制台',
    announcement: '公告',
    settings: '设置'
  },

  // 主页
  home: {
    title: '主页',
    coreStatus: '核心状态',
    notRunning: '未运行',
    running: '核心运行中',
    taskRunning: '任务执行中',
    error: '核心异常',
    startTask: '开始任务',
    stopTask: '停止任务',
    restartCore: '重启核心',
    startCore: '启动核心',
    configList: '配置列表',
    selectAll: '全选',
    deselectAll: '取消全选',
    invertSelection: '反选',
    deleteSelected: '删除选中',
    newConfig: '新建配置',
    refresh: '刷新',
    configName: '配置名称',
    create: '创建',
    cancel: '取消',
    configManagement: '配置管理',
    quickAccess: '快速访问',
    versionUpdate: '版本更新',
    versionUpdateDesc: '查看最新版本信息',
    announcement: '公告',
    announcementDesc: '查看最新公告',
    tutorial: '教程文档',
    tutorialDesc: '查看使用教程',
    notifications: {
      cannotDeleteDefault: '默认配置项禁止删除。',
      taskStopped: '任务已停止',
      stopTaskFailed: '停止任务失败',
      selectConfigFirst: '请先选择要执行的配置',
      taskStarted: '开始执行任务: {names}',
      executeTaskFailed: '执行任务失败',
      taskCompleted: '任务执行完成',
      coreRestarted: '核心重启成功',
      coreStarted: '核心启动成功',
      coreRestartFailed: '核心重启失败，请检查SRA-cli是否存在或手动排查问题'
    }
  },

  // 任务页面
  tasks: {
    title: '任务',
    selectConfig: '选择配置',
    saveConfig: '保存配置',
    newConfig: '新建配置',
    saveAndExecute: '保存并执行',
    configName: '配置名称',
    configNamePlaceholder: '请输入配置名称',
    cancel: '取消',
    confirm: '确定',
    
    // 启动游戏
    startGame: {
      title: '启动游戏',
      gamePath: '游戏路径',
      selectPath: '选择路径',
      channel: '渠道',
      officialServer: '官服',
      bilibiliServer: 'B服',
      autoLogin: '自动登录',
      autoLoginHint: '勾选自动登录即表明你已充分阅读并同意《用户协议》和《隐私政策》',
      alwaysLogin: '每次都登录',
      username: '用户名',
      usernamePlaceholder: '请输入用户名',
      password: '密码',
      passwordPlaceholder: '请输入密码'
    },

    // 清体力
    trailblazePower: {
      title: '清体力',
      taskList: '任务列表',
      addTask: '添加任务',
      selectLevel: '选择关卡',
      continuousBattle: '持续作战',
      runTimes: '执行次数',
      add: '添加',
      times: '次数',
      useSupport: '使用支援角色',
      useSkill: '使用战技',
      replenishPower: '补充体力',
      replenishTimes: '补充次数',
      replenishMethod: '补充方式',
      reservedPower: '后备开拓力',
      fuel: '燃料',
      stellarJade: '星琼',
      changeLineup: '更换队伍',
      lineupCheck: '队伍检查',
      reLineup: '重新编队',
      useSkillEntry: '使用战技入场',
      selectInstance: '---选择副本---',
      
      // 关卡类型
      categories: {
        calyxGolden: '拟造花萼（金）',
        calyxCrimson: '拟造花萼（赤）',
        stagnantShadow: '凝滞虚影',
        cavern: '侵蚀隧洞',
        echoOfWar: '历战余响',
        ornament: '饰品提取'
      }
    },

    // 清体力关卡名称
    trailblazePowerLevels: {
      // 饰品提取
      ornament: {
        moonlitBlood: '月下朱殷（妖精/海隅）',
        unceasingStrife: '纷争不休（拾骨地/巨树）',
        famishedWorker: '蠹役饥肠（露莎卡/蕉乐园）',
        eternalComedy: '永恒笑剧（都蓝/劫火）',
        toSweetDreams: '伴你入眠（茨冈尼亚/出云显世）',
        pouringBlades: '天剑如雨（格拉默/匹诺康尼）',
        fruitOfEvil: '孽果盘生（繁星/龙骨）',
        permafrost: '百年冻土（贝洛伯格/萨尔索图）',
        gentleWords: '温柔话语（公司/差分机）',
        smeltedHeart: '浴火钢心（塔利亚/翁瓦克）',
        untoppledWalls: '坚城不倒（太空封印站/仙舟）'
      },
      // 拟造花萼（金）
      calyxGolden: {
        budOfMemoriesAmphoreus: '回忆之蕾（翁法罗斯）',
        budOfAetherAmphoreus: '以太之蕾（翁法罗斯）',
        budOfTreasuresAmphoreus: '珍藏之蕾（翁法罗斯）',
        budOfMemoriesPenacony: '回忆之蕾（匹诺康尼）',
        budOfAetherPenacony: '以太之蕾（匹诺康尼）',
        budOfTreasuresPenacony: '珍藏之蕾（匹诺康尼）',
        budOfMemoriesLuofu: '回忆之蕾（仙舟罗浮）',
        budOfAetherLuofu: '以太之蕾（仙舟罗浮）',
        budOfTreasuresLuofu: '珍藏之蕾（仙舟罗浮）',
        budOfMemoriesJarilo: '回忆之蕾（雅利洛VI）',
        budOfAetherJarilo: '以太之蕾（雅利洛VI）',
        budOfTreasuresJarilo: '珍藏之蕾（雅利洛VI）'
      },
      // 拟造花萼（赤）
      calyxCrimson: {
        moonRageFang: '月狂獠牙（毁灭）',
        worldbreakerBlade: '净世残刃（毁灭）',
        divineAmber: '神体琥珀（存护）',
        safeguardOfAmber: '琥珀的坚守（存护）',
        countertemporalShot: '逆时一击（巡猎）',
        arrowOfTheStarchaser: '逐星之矢（巡猎）',
        myriadFruit: '万象果实（丰饶）',
        flowerOfEternity: '永恒之花（丰饶）',
        exquisiteColoredDraft: '精致色稿（智识）',
        keyOfWisdom: '智识之钥（智识）',
        heavenlyMelody: '天外乐章（同谐）',
        stellarisSymphony: '群星乐章（同谐）',
        heavenIncinerator: '焚天之魔（虚无）',
        obsidianOfObsession: '沉沦黑曜（虚无）',
        flowerOfAlaya: '阿赖耶华（记忆）'
      },
      // 凝滞虚影
      stagnantShadow: {
        invasiveClot: '侵略凝块（物理）',
        ipcWorkPermit: '星际和平工作证（物理）',
        netherworldToken: '幽府通令（物理）',
        brokenTeethOfIronWolf: '铁狼碎齿（物理）',
        radiantProminence: '怠火之心（火）',
        ragingHeart: '过热钢刀（火）',
        searingSteelBlade: '恒温晶壳（火）',
        endothermChitin: '海妖残鰭（火）',
        seaSirensTornFin: '冷藏梦箱（冰）',
        dreamFridge: '苦寒晶壳（冰）',
        gelidChitin: '风雪之角（冰）',
        hornOfSnow: '兽馆之钉（冰）',
        nailOfTheBeastCoffin: '炼形者雷枝（雷）',
        shapeShiffersLightningStaff: '往日之影的雷冠（雷）',
        lightningCrownOfThePastShadow: '暮辉烬蕾（雷）',
        charredBudOfTwilight: '一杯酪酊的时代（风）',
        aGlassOfTheBesottedEra: '无人遗垢（风）',
        ascendantDebirs: '暴风之眼（风）',
        stormEye: '暗帷月华（风）',
        darkveilMoonlight: '炙梦喷枪（量子）',
        dreamFlamer: '苍猿之钉（量子）',
        nailOfTheApe: '虚幻铸铁（量子）',
        voidCastIron: '纷争前兆（虚数）',
        harbingerOfStrife: '一曲和弦的幻景（虚数）',
        chordalMirage: '镇灵敕符（虚数）',
        suppressingEdict: '往日之影的金饰（虚数）',
        goldenCrownOfThePastShadow: '往日之影的金饰（虚数）'
      },
      // 侵蚀隧洞
      cavern: {
        pathOfHiddenSalvation: '隐救之径（救世主/隐士）',
        pathOfThundersurge: '雳勇之径（女武神/船长）',
        pathOfAria: '弦歌之径（英豪/诗人）',
        pathOfUncertainty: '迷识之径（司铎/学者）',
        pathOfCavalier: '勇骑之径（铁骑/勇烈）',
        pathOfDreamdive: '梦潜之径（死水/钟表匠）',
        pathOfDarkness: '幽冥之径（大公/幽囚）',
        pathOfElixirSeekers: '药使之径（莳者/信使）',
        pathOfConflagration: '野焰之径（火匠/废土客）',
        pathOfHolyHymn: '圣颂之径（圣骑/乐队）',
        pathOfProvidence: '睿智之径（铁卫/量子）',
        pathOfDrifting: '漂泊之径（过客/快枪手）',
        pathOfJabbingPunch: '迅拳之径（拳皇/怪盗）',
        pathOfGelidWind: '霜风之径（冰/风）'
      },
      // 历战余响
      echoOfWar: {
        glanceOfTwilight: '晨昏的回眸',
        innerBeastsBattlefield: '心兽的战场',
        salutationsOfAshenDreams: '尘梦的赞礼',
        boreholePlanetsPastNightmares: '蛀星的旧靥',
        divineSeed: '不死的神实',
        endOfTheEternalFreeze: '寒潮的落幕',
        destructionsBeginning: '毁灭的开端',
        rustedCryptOfTheIronCarcass: '铁骸的锈冢'
      }
    },

    // 领取奖励
    receiveReward: {
      title: '领取奖励',
      guide: '星际和平指南',
      assignments: '委托',
      mailbox: '邮箱',
      dailyTraining: '每日实训',
      namelessHonor: '无名勋礼',
      giftOfOdyssey: '巡星之礼',
      redemptionCode: '兑换码',
      redeemCodes: '兑换码列表',
      redeemCodesPlaceholder: '每行一个兑换码'
    },

    // 旷宇纷争
    cosmicStrife: {
      title: '旷宇纷争',
      enable: '启用',
      mode: '模式',
      clearLevel: '通关',
      divergentUniverse: '差分宇宙',
      currencyWars: '货币战争',
      policy: '策略',
      proteanHero: '差分宇宙 - 千面英雄',
      singleLayer: '刷单层',
      standardGambit: '标准博弈',
      overclockGambit: '超频博弈',
      runTimes: '运行次数',
      trailblazer: '开拓者',
      trailblazerPlaceholder: '请输入开拓者名称',
      zeroSumGame: '零和博弈',
      hintText: '请先手动完成一次以清除所有新手提示'
    },

    // 完成任务后
    afterMission: {
      title: '完成任务后',
      exitGame: '退出游戏',
      logout: '登出',
      sleep: '休眠',
      shutdown: '关机',
      exitApp: '退出应用',
      none: '无动作'
    },

    notifications: {
      configSaved: '配置保存成功',
      configSaveFailed: '配置保存失败',
      taskStarted: '开始执行任务: {name}',
      executeFailed: '执行失败',
      configCreated: '配置创建成功',
      configCreateFailed: '配置创建失败',
      selectLevel: '请选择关卡'
    },

    // 任务列表相关
    taskList: {
      singleTime: '单次',
      execution: '执行',
      moveUp: '上移',
      moveDown: '下移',
      deleteTask: '删除任务'
    },

    // 任务类型名称
    taskTypes: {
      ornament: '饰品提取',
      calyxGolden: '拟造花萼（金）',
      calyxCrimson: '拟造花萼（赤）',
      stagnantShadow: '凝滞虚影',
      cavern: '侵蚀隧洞',
      echoOfWar: '历战余响'
    }
  },

  // 扩展页面
  extensions: {
    title: '扩展',
    autoPlot: {
      title: '自动对话',
      enable: '启用',
      enableDesc: '启用自动对话功能',
      skipPlot: '跳过对话',
      skipPlotDesc: '自动跳过对话内容'
    },
    notifications: {
      autoPlotEnabled: '已启用自动对话',
      autoPlotDisabled: '已禁用自动对话',
      skipPlotEnabled: '已启用跳过对话',
      skipPlotDisabled: '已禁用跳过对话',
      toggleFailed: '切换失败'
    }
  },

  // 控制台
  console: {
    title: '控制台',
    filterLevel: '过滤级别',
    filterSource: '过滤来源',
    clearConsole: '清空控制台',
    exportLogs: '导出日志',
    inputPlaceholder: '输入命令...',
    send: '发送',
    sources: {
      frontend: '前端',
      backend: '后端',
      process: '进程端'
    },
    notifications: {
      noLogsToExport: '没有日志可导出',
      exportSuccess: '日志导出成功',
      exportFailed: '日志导出失败',
      taskStarted: '开始执行自定义任务'
    },
    sendFailed: '发送失败',
    sendMessageFailed: '发送消息失败',
    logs: {
      wallpaperDataLength: '壁纸数据长度: {length}',
      startGetWallpaper: '开始调用 get_wallpaper_base64',
      getWallpaperResult: 'get_wallpaper_base64 返回: {result}',
      hasData: '有数据',
      noData: '无数据',
      loadWallpaperSuccess: '应用启动时加载壁纸成功',
      noCustomWallpaper: '没有自定义壁纸，使用默认背景',
      loadWallpaperFailed: '加载壁纸失败: {error}',
      loadSettingsFailed: '加载设置失败: {error}',
      openWallpaperDialog: '打开壁纸选择对话框',
      userCancelledWallpaper: '用户取消了壁纸选择',
      userSelectedWallpaper: '用户选择了壁纸: {path}',
      callSetWallpaper: '调用 set_wallpaper 命令',
      wallpaperCached: '壁纸已缓存到: {path}',
      callGetWallpaperBase64: '调用 get_wallpaper_base64 命令',
      getWallpaperBase64Result: 'get_wallpaper_base64 返回: {result}',
      hasDataWithSize: '有数据({size}字节)',
      startApplyWallpaper: '开始应用壁纸到背景',
      wallpaperApplied: '壁纸已应用到背景',
      getWallpaperBase64Empty: 'get_wallpaper_base64 返回空数据',
      startResetWallpaper: '开始重置壁纸',
      wallpaperStyleApplied: '壁纸样式已应用',
      appContainerNotFound: '找不到.app-container元素',
      wallpaperStyleRemoved: '壁纸样式已移除'
    }
  },

  // 公告
  announcement: {
    title: '公告',
    refresh: '刷新',
    mainAnnouncement: '主要公告',
    releaseAnnouncement: '最新版本公告',
    otherAnnouncements: '其他公告',
    loading: '加载中...',
    error: '公告获取失败：如果你没挂梯子，那这不是你的问题，这可能是由于服务器未运行或被攻击。',
    retry: '重试',
    noAnnouncements: '暂无公告'
  },

  // 设置
  settings: {
    title: '设置',
    saveSettings: '保存设置',
    resetSettings: '重置设置',
    
    theme: {
      title: '主题设置',
      customWallpaper: '自定义壁纸',
      selectWallpaper: '选择新壁纸',
      resetWallpaper: '重置壁纸'
    },
    
    interface: {
      title: '界面设置',
      language: '界面语言',
      languageNote: '使用该程序进行自动化需要您游戏界面设置为简体中文',
      zoom: '屏幕缩放比例',
      confidence: '识图置信度阈值'
    },
    
    hotkeys: {
      title: '游戏内快捷键',
      clickToSet: '点击输入框设置快捷键',
      pressKey: '按键盘设置快捷键',
      activity: '活动',
      chronicle: '纪行',
      warp: '卡池',
      guide: '指南',
      map: '地图',
      technique: '秘技'
    },
    
    notifications: {
      title: '通知',
      allowNotifications: '允许通知',
      allowSystemNotifications: '系统通知',
      allowEmailNotifications: '邮件通知',
      smtpSettings: 'SMTP设置',
      smtpServer: 'SMTP服务器',
      smtpPort: 'SMTP端口',
      emailSender: '发送邮件地址',
      emailSenderPlaceholder: '例：123456@qq.com',
      emailAuthCode: '邮件授权码',
      emailAuthCodePlaceholder: '例：123456',
      emailReceiver: '接收邮件地址',
      emailReceiverPlaceholder: '例：123456@qq.com',
      settingsSaved: '设置保存成功',
      saveSettingsFailed: '保存设置失败',
      wallpaperSet: '壁纸设置成功',
      selectWallpaperFailed: '选择壁纸失败',
      wallpaperReset: '壁纸已重置为默认',
      resetWallpaperFailed: '重置壁纸失败',
      unknownError: '未知错误'
    },
    
    app: {
      title: '应用设置',
      enableStartupLaunch: '开机自启动',
      enableMinimizeToTray: '最小化到托盘'
    },
    
    resetConfirm: '确定要重置所有设置为默认值吗？'
  },

  // 通用
  common: {
    confirm: '确认',
    cancel: '取消',
    save: '保存',
    delete: '删除',
    edit: '编辑',
    close: '关闭',
    loading: '加载中...',
    success: '成功',
    error: '错误',
    warning: '警告'
  }
}
