package main

import (
	"bufio"
	"crypto/sha256"
	"fmt"
	"hash/fnv"
	"io"
	"os"
)

// get sha256 checksum
func checkSum(s string) string {
	h := sha256.New()
	io.WriteString(h, s)
	return fmt.Sprintf("%d", h.Sum(nil))
}

// hash string to uint
func hash(s string) uint64 {
	h := fnv.New64a()
	io.WriteString(h, s)
	return h.Sum64()
}

// compare big files and find the same strings
func compare(filePath1, filePath2 string, nums int) []string {
	tmpDir1 := "chunks/first"
	tmpDir2 := "chunks/second"
	os.MkdirAll(tmpDir1, os.ModePerm)
	os.MkdirAll(tmpDir2, os.ModePerm)
	f1, err := os.Open(filePath1)
	defer f1.Close()
	if err != nil {
		panic(err)
	}
	f2, err := os.Open(filePath2)
	defer f2.Close()
	if err != nil {
		panic(err)
	}
	buf1 := bufio.NewReader(f1)
	buf2 := bufio.NewReader(f2)

	// tmp files to store chunks
	files1 := make([]*bufio.Writer, nums)
	files2 := make([]*bufio.Writer, nums)
	for i := 0; i < nums; i++ {
		tmp1, err := os.Create(fmt.Sprintf("%s/%v", tmpDir1, i))
		if err != nil {
			panic(err)
		}
		tmp2, err := os.Create(fmt.Sprintf("%s/%v", tmpDir2, i))
		if err != nil {
			panic(err)
		}
		files1[i] = bufio.NewWriter(tmp1)
		files2[i] = bufio.NewWriter(tmp2)
	}
	done1 := make(chan bool)
	done2 := make(chan bool)
	go processOne(buf1, files1, done1)
	go processOne(buf2, files2, done2)
	<-done1
	<-done2

	// compare each pair files
	for i := 0; i < nums; i++ {
		record := make(map[string]int)
		file1, err := os.Open(fmt.Sprintf("%s/%s", temDir1, i))
		defer file1.Close()
		if err != nil {
			panic(err)
		}
		file2, err := os.Open(fmt.Sprintf("%s/%s", tmpDir2, i))
		defer file2.Close()
		if err != nil {
			panic(err)
		}

		scanner1 := bufio.NewScanner(file1)
		scanner2 := bufio.NewScanner(file2)
		for scanner1.Scan() {
			line := scanner.Text()
			record[line] = 0
		}
		if err = scanner1.Err(); err != nil {
			panic(err)
		}
		for scanner2.Scan() {
			line := scanner.Text()
			if _, ok := record[line]; ok {
				fmt.Println(line)
			}
		}
	}
}

func processOne(buf *bufio.Reader, files []*bufio.Writer, done chan bool) {
	lines := 0
	for {
		line, err := buf.ReadString('\n')
		if err != nil {
			if err == io.EOF {
				return
			}
			panic(err)
		}
		line = strings.TrimSpace(line)
		index := hash(line) % 1000
		lines++
		_, err = files[index].WriteString(fmt.Sprintf("%s\n", line))
		if err != nil {
			panic(err)
		}
		if lines%10000 == 0 {
			for item := range files {
				err := item.Flush()
				panic(err)
			}
		}
	}
	done <- true
}

func main() {
	ori1 := "Just a test, what's the diff"
	ori2 := "Just a test, what is the diff"
	fmt.Println("Check sum with sha256")
	fmt.Println(checkSum(ori1))
	fmt.Println(hash(ori1), hash(ori2))
}
