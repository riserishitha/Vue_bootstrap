<template>
  <div>
    <h2 class="text-center mb-4">Vue To-Do App (CRUD)</h2>
    <div class="input-group mb-3">
      <input type="text" class="form-control" placeholder="Enter task" v-model="task" @keyup.enter="addOrUpdateTask" />
      <button class="btn btn-primary" @click="addOrUpdateTask">
        {{ isEdit ? "Update" : "Add" }}
      </button>
    </div>
    <ul class="list-group">
      <li v-for="(item, index) in tasks" :key="index" class="list-group-item d-flex justify-content-between align-items-center">
        <span>{{ item }}</span>
        <div>
          <button class="btn btn-sm btn-warning me-2" @click="editTask(index)">Edit</button>
          <button class="btn btn-sm btn-danger" @click="deleteTask(index)">Delete</button>
        </div>
      </li>
    </ul>
  </div>
</template>
<script>
export default {
  data() {
    return {
      task: '',
      tasks: [],
      isEdit: false,
      editIndex: null
    }
  },
  mounted() {
    const stored = localStorage.getItem('tasks')
    if (stored) this.tasks = JSON.parse(stored)
  },
  methods: {
    addOrUpdateTask() {
      if (this.task.trim() === '') return

      if (this.isEdit) {
        this.tasks[this.editIndex] = this.task
        this.isEdit = false
        this.editIndex = null
      } else {
        this.tasks.push(this.task)
      }

      this.task = ''
      this.saveTasks()
    },
    editTask(index) {
      this.task = this.tasks[index]
      this.isEdit = true
      this.editIndex = index
    },
    deleteTask(index) {
      this.tasks.splice(index, 1)
      this.saveTasks()
    },
    saveTasks() {
      localStorage.setItem('tasks', JSON.stringify(this.tasks))
    }
  }
}
</script>
<style scoped>
input {
  border-radius: 0.375rem;
}
</style>
