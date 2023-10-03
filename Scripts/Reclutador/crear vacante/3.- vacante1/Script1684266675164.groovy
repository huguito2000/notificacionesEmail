import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import java.nio.file.WatchService as WatchService
import javax.xml.bind.annotation.XmlElementDecl.GLOBAL as GLOBAL
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.keyword.builtin.GetResponseStatusCodeKeyword as GetResponseStatusCodeKeyword
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import com.kms.katalon.entity.global.GlobalVariableEntity as GlobalVariableEntity
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonOutput as JsonOutput
import groovy.json.JsonSlurper as JsonSlurper
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject

min = 0
max = 0
salaryShow = 0

WebUI.callTestCase(findTestCase('reclutador/crear vacante/2.- crear vacante'), [:], FailureHandling.STOP_ON_FAILURE)

SalarioMin()
min  = min as int

GlobalVariable.SalarioMin = min

println(GlobalVariable.SalarioMin)

SalarioMax()

GlobalVariable.SalarioMax = max

println(GlobalVariable.SalarioMax)

SalarioShow()

println(salaryShow)

if(salaryShow == 1) {
	GlobalVariable.salarioShow = salarioShow = false
}else {
	GlobalVariable.salarioShow = true
}

SalarioShow()

println(salaryShow)

if(salaryShow == 1) {
	GlobalVariable.confidencial = salarioShow = true
}else {
	GlobalVariable.confidencial = false
}

println(GlobalVariable.confidencial )

println(GlobalVariable.salarioShow)
response = WS.sendRequest(findTestObject('Reclutador/crear vacantes/vacante manual/3.- vacante1'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

responseText = response.getResponseText()

println(responseText)

WS.verifyResponseStatusCode(response, 200)

responseText = response.getResponseText()

println(responseText)

json = new JsonSlurper().parseText(responseText)

json = json.vacantId

println(json)

GlobalVariable.vacanteid = json

println(json)

def SalarioShow() {
	salaryShow = ((Math.random() * 3) as int)
	}


def SalarioMin() {
	min = ((Math.random() * 1000) as int)
}

def SalarioMax() {
	max = ((Math.random() * 30000) as int)
	}
