import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Announcement from '../views/Announcement.vue'
import AnnouncementWindow from '../views/AnnouncementWindow.vue'
import VersionUpdateWindow from '../views/VersionUpdateWindow.vue'
import Tasks from '../views/Tasks.vue'
import Extensions from '../views/Extensions.vue'
import Console from '../views/Console.vue'
import Settings from '../views/Settings.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/announcement',
    name: 'Announcement',
    component: Announcement
  },
  {
    path: '/announcement-window',
    name: 'AnnouncementWindow',
    component: AnnouncementWindow
  },
  {
    path: '/version-update',
    name: 'VersionUpdate',
    component: VersionUpdateWindow
  },
  {
    path: '/tasks',
    name: 'Tasks',
    component: Tasks
  },
  {
    path: '/extensions',
    name: 'Extensions',
    component: Extensions
  },
  {
    path: '/console',
    name: 'Console',
    component: Console
  },
  {
    path: '/settings',
    name: 'Settings',
    component: Settings
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router