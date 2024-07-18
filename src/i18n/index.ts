import { createI18n } from 'vue-i18n';

const messages = {
  en: {
    generateCharacter: 'Generate Character',
    characterInfo: 'Character Information',
    name: 'Name',
    gender: 'Gender',
    age: 'Age',
    strength: 'Strength',
    agility: 'Agility',
    charisma: 'Charisma',
    intelligence: 'Intelligence',
    currentTime: 'Current Time',
    pause: 'Pause',
    resume: 'Resume',
    travel: 'Travel',
    team: 'Team',
    travelContent: 'Travel content goes here...',
    teamContent: 'Team content goes here...',
    switchToDarkMode: 'Switch to Dark Mode',
    switchToLightMode: 'Switch to Light Mode',
    walkingSpeed: 'Walking Speed',
    ridingSpeed: 'Riding Speed',
    isRiding: 'Is Riding',
    yes: 'Yes',
    no: 'No',
    travelDistance: 'Travel Distance',
    distanceUnit: 'm',
    teamSpeed: 'Team Speed'
  },
  zh: {
    generateCharacter: '生成角色',
    characterInfo: '角色信息',
    name: '名字',
    gender: '性别',
    age: '年龄',
    strength: '力量',
    agility: '敏捷',
    charisma: '魅力',
    intelligence: '智力',
    currentTime: '当前时间',
    pause: '暂停',
    resume: '继续',
    travel: '旅行',
    team: '团队',
    travelContent: '旅行内容在此...',
    teamContent: '团队内容在此...',
    switchToDarkMode: '切换到暗色模式',
    switchToLightMode: '切换到亮色模式',
    walkingSpeed: '步行速度',
    ridingSpeed: '骑马速度',
    isRiding: '是否骑行',
    yes: '是',
    no: '否',
    travelDistance: '旅行距离',
    distanceUnit: '米',
    teamSpeed: '团队速度'
  }
};

const i18n = createI18n({
  locale: 'en',
  messages
});

export default i18n;
