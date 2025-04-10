<template>
  <div class="container mt-4">
    <h2 class="text-center mb-4">Vue To-Do App (Connected to Rust Backend)</h2>
    <div class="input-group mb-3">
      <input
        type="text"
        class="form-control"
        placeholder="Enter task"
        v-model="task"
        @keyup.enter="addOrUpdateTask"
      />
      <button class="btn btn-primary" @click="addOrUpdateTask">
        {{ isEdit ? "Update" : "Add" }}
      </button>
    </div>

    <ul class="list-group">
      <li
        v-for="(item, index) in tasks"
        :key="item._id"
        class="list-group-item d-flex justify-content-between align-items-center"
      >
        <span>{{ item.name }}</span>
        <div>
          <button class="btn btn-sm btn-warning me-2" @click="editTask(index)">Edit</button>
          <button class="btn btn-sm btn-danger" @click="deleteTask(item._id)">Delete</button>
        </div>
      </li>
    </ul>
  </div>
</template>

<script>
import axios from "axios"

export default {
  name: "TodoApp",
  data() {
    return {
      task: "",
      tasks: [],
      isEdit: false,
      editTaskId: null,
    }
  },
  methods: {
    async fetchTasks() {
      try {
        const res = await axios.get("http://localhost:8080/tasks")
        this.tasks = res.data
      } catch (err) {
        console.error("Failed to fetch tasks:", err)
      }
    },
    async addOrUpdateTask() {
      if (!this.task.trim()) return

      try {
        if (this.isEdit) {
          await axios.put(`http://localhost:8080/tasks/${this.editTaskId}`, {
            name: this.task,
          })
          this.isEdit = false
          this.editTaskId = null
        } else {
          await axios.post("http://localhost:8080/tasks", {
            name: this.task,
          })
        }
        this.task = ""
        this.fetchTasks()
      } catch (err) {
        console.error("Error adding/updating task:", err)
      }
    },
    editTask(index) {
      this.task = this.tasks[index].name
      this.isEdit = true
      this.editTaskId = this.tasks[index]._id
    },
    async deleteTask(id) {
      try {
        await axios.delete(`http://localhost:8080/tasks/${id}`)
        this.fetchTasks()
      } catch (err) {
        console.error("Error deleting task:", err)
      }
    },
  },
  mounted() {
    this.fetchTasks()
  },
}
</script>

<style scoped>
input {
  border-radius: 0.375rem;
}
</style>
